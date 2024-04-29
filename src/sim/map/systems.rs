use bevy::prelude::*;
use bevy_ecs_tilemap::{helpers::hex_grid::offset::*, prelude::*};
use noise::{NoiseFn, Perlin};
use rand::Rng;

use super::{
    components::{ChunkPos, TerrainType, Tile, TileIndex},
    BaseSpawnEvent, CHUNK_MAP_SIDE_LENGTH_X, CHUNK_MAP_SIDE_LENGTH_Y, GRID_SIZE_HEX_ROW, MAP_SIZE,
    SEED, TERRAIN_SPRITE_PATH, TILE_SIZE_HEX_ROW,
};

const MAP_TYPE: TilemapType = TilemapType::Hexagon(HexCoordSystem::RowEven);

fn chunk_in_world_position(pos: IVec2) -> Vec3 {
    Vec3::new(
        TILE_SIZE_HEX_ROW.x * CHUNK_MAP_SIDE_LENGTH_X as f32 * pos.x as f32,
        TilePos {
            x: 0,
            y: CHUNK_MAP_SIDE_LENGTH_Y,
        }
        .center_in_world(&GRID_SIZE_HEX_ROW, &MAP_TYPE)
        .y * pos.y as f32,
        0.0,
    )
}

fn hex_pos_from_tile_pos(
    tile_pos: &TilePos,
    grid_size: &TilemapGridSize,
    map_transform: &Transform,
) -> IVec2 {
    let tile_translation =
        *map_transform * tile_pos.center_in_world(grid_size, &MAP_TYPE).extend(0.0);

    let pos = RowEvenPos::from_world_pos(&tile_translation.truncate(), grid_size);
    IVec2 { x: pos.q, y: pos.r }
}

fn tile_to_world_pos(
    tile_pos: &TilePos,
    grid_size: &TilemapGridSize,
    map_transform: &Transform,
) -> Vec2 {
    let tile_translation =
        *map_transform * tile_pos.center_in_world(grid_size, &MAP_TYPE).extend(0.0);

    tile_translation.truncate()
}

pub fn despawn_map(
    mut commands: Commands,
    mut tilemap_query: Query<Entity, With<TileStorage>>,
    mut text_q: Query<Entity, With<Text>>,
) {
    for entity in tilemap_query.iter_mut() {
        commands.entity(entity).despawn_recursive();
    }

    for entity in text_q.iter_mut() {
        commands.entity(entity).despawn_recursive();
    }
}

#[derive(SystemSet, Clone, Copy, Hash, PartialEq, Eq, Debug)]
pub struct SpawnChunksSet;

fn spawn_first_chunk(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    noise: Perlin,
    base_spawned_ew: EventWriter<BaseSpawnEvent>,
) {
    let chunk_pos = ChunkPos(IVec2 { x: 0, y: 0 });
    spawn_chunk(commands, asset_server, noise, chunk_pos, base_spawned_ew)
}

pub fn spawn_chunk(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    noise: Perlin,
    chunk_pos: ChunkPos,
    base_spawned_ew: EventWriter<BaseSpawnEvent>,
) {
    let tiles = fill_tile_chunk(noise, chunk_pos, base_spawned_ew);

    let mut tile_storage = TileStorage::empty(MAP_SIZE);
    let tilemap_entity = commands.spawn_empty().id();
    let tilemap_id = TilemapId(tilemap_entity);

    fill_chunk(tiles, tilemap_id, commands, &mut tile_storage);

    let texture_handle: Handle<Image> = asset_server.load(TERRAIN_SPRITE_PATH);
    let chunk_position = chunk_in_world_position(*chunk_pos);
    commands
        .entity(tilemap_entity)
        .insert((
            TilemapBundle {
                grid_size: GRID_SIZE_HEX_ROW,
                size: MAP_SIZE,
                storage: tile_storage,
                texture: TilemapTexture::Single(texture_handle),

                tile_size: TILE_SIZE_HEX_ROW,
                map_type: MAP_TYPE,
                transform: Transform::from_translation(chunk_position),
                ..Default::default()
            },
            Name::new(format!("Chunk: {:?}", *chunk_pos)),
        ))
        .insert(chunk_pos);
}

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    base_spawned_ew: EventWriter<BaseSpawnEvent>,
) {
    let mut rng = rand::thread_rng();
    let seed = if SEED == 0 { rng.gen() } else { SEED };
    let noise = Perlin::new(seed);
    spawn_first_chunk(&mut commands, &asset_server, noise, base_spawned_ew);
}

pub fn get_noise_value(
    noise: Perlin,
    divider: f64,
    tile_pos: TilePos,
    chunk_offset: ChunkPos,
) -> f64 {
    let x = tile_pos.x as f64;
    let y = tile_pos.y as f64;
    let random = noise.get([
        (x + (chunk_offset.x as f64 * CHUNK_MAP_SIDE_LENGTH_X as f64)) / divider,
        (y + (chunk_offset.y as f64 * CHUNK_MAP_SIDE_LENGTH_Y as f64)) / divider,
    ]);

    random
}

fn terrain_type(moist: f64, temp: f64) -> TerrainType {
    match () {
        _ if 0.0 <= moist && moist <= 1.0 && 0.0 <= temp && temp <= 0.2 => TerrainType::Snow,
        _ if 0.0 <= moist && moist <= 0.4 && 0.0 <= temp && temp <= 0.5 => TerrainType::Mushroom,
        _ if 0.0 <= moist && moist <= 0.4 && 0.5 <= temp && temp <= 1.0 => TerrainType::Desert,
        _ if 0.5 <= moist && moist <= 0.9 && 0.4 <= temp && temp <= 1.0 => TerrainType::Jungle,
        _ if 0.9 <= moist && moist <= 1.0 && 0.0 <= temp && temp <= 1.0 => TerrainType::Lake,
        _ if 0.0 <= moist && moist <= 0.9 && 0.2 <= temp && temp <= 0.4 => TerrainType::Grassland,
        _ => TerrainType::Rocky,
    }
}

fn fill_tile_chunk(
    noise: Perlin,
    chunk_offset: ChunkPos,
    mut base_spawned_ew: EventWriter<BaseSpawnEvent>,
) -> Vec<Tile> {
    let mut tiles: Vec<Tile> = vec![];

    let mut base_spawned = false;
    for x in 0..CHUNK_MAP_SIDE_LENGTH_X {
        for y in 0..CHUNK_MAP_SIDE_LENGTH_Y {
            let pos = TilePos::new(x, y);
            let alt = get_noise_value(noise, 100.5, pos, chunk_offset);
            let moist = get_noise_value(noise, 63.5, pos, chunk_offset);
            let temp = get_noise_value(noise, 33.5, pos, chunk_offset);

            let noise_val = (alt + moist + temp) / 3.0;

            // Ocean
            if noise_val < 0. {
                tiles.push(Tile {
                    pos,
                    index: TileIndex::WaterOcean,
                });
                continue;
            }

            // Beach
            if noise_val < 0.05 {
                tiles.push(Tile {
                    pos,
                    index: TileIndex::Desert,
                });
                continue;
            }

            // Other biomes
            if noise_val < 0.7 {
                match terrain_type(moist, temp) {
                    TerrainType::Jungle => {
                        tiles.push(Tile {
                            pos,
                            index: TileIndex::Jungle,
                        });
                    }
                    TerrainType::Desert => {
                        tiles.push(Tile {
                            pos,
                            index: TileIndex::Desert,
                        });
                    }
                    TerrainType::Mushroom => {
                        tiles.push(Tile {
                            pos,
                            index: TileIndex::Mushroom,
                        });
                    }
                    TerrainType::Snow => {
                        tiles.push(Tile {
                            pos,
                            index: TileIndex::Snow,
                        });
                    }
                    _ => {
                        if TerrainType::Lake == terrain_type(moist, temp) {
                            tiles.push(Tile {
                                pos,
                                index: TileIndex::WaterLake,
                            });
                        } else {
                            if !base_spawned {
                                let pos = tile_to_world_pos(
                                    &pos,
                                    &GRID_SIZE_HEX_ROW,
                                    &Transform::default(),
                                );
                                base_spawned_ew.send(BaseSpawnEvent { position: pos });
                                base_spawned = true;
                                continue;
                            }
                            tiles.push(Tile {
                                pos,
                                index: TileIndex::Grass,
                            });
                        }
                    }
                }
                continue;
            }

            tiles.push(Tile {
                pos,
                index: TileIndex::Rock,
            });
        }
    }

    tiles
}

/// Fills an entire tile storage with the given tiles.
pub fn fill_chunk(
    tiles: Vec<Tile>,
    tilemap_id: TilemapId,
    commands: &mut Commands,
    tile_storage: &mut TileStorage,
) {
    commands.entity(tilemap_id.0).with_children(|parent| {
        for tile in tiles {
            let tile_entity = parent
                .spawn((
                    TileBundle {
                        position: tile.pos,
                        tilemap_id,
                        texture_index: TileTextureIndex(tile.index as u32),
                        ..Default::default()
                    },
                    Name::new(format!("Tile {:?}", tile.pos)),
                ))
                .id();
            tile_storage.set(&tile.pos, tile_entity);
        }
    });
}
