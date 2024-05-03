use bevy::prelude::*;
use bevy_ecs_tilemap::{helpers::hex_grid::offset::*, prelude::*};
use noise::{NoiseFn, Perlin};
use rand::Rng;

use crate::sim::{
    droids::components::Robot,
    resources::{
        components::{Resource, ResourceType},
        events::ResourceSpawnEvent,
    },
};

use super::{
    components::{ChunkPos, MapManager, TerrainType, Tile, TileIndex, BIOME_DATA, RESOURCES_DATA},
    BaseSpawnEvent, CHUNK_MAP_SIDE_LENGTH_X, CHUNK_MAP_SIDE_LENGTH_Y, GRID_SIZE_HEX_ROW, MAP_SIZE,
    TERRAIN_SPRITE_PATH, TILE_HEIGHT, TILE_SIZE_HEX_ROW, TILE_WIDTH,
};

const MAP_TYPE: TilemapType = TilemapType::Hexagon(HexCoordSystem::RowEven);

// Altitude values
const DEEP_WATER_LEVEL: f64 = -0.15;
const SHALLOW_WATER_LEVEL: f64 = -0.1;
const SHORE_LEVEL: f64 = -0.05;
const MOUNTAIN_LEVEL: f64 = 0.7;

pub fn despawn_map(mut commands: Commands, mut tilemap_query: Query<Entity, With<TileStorage>>) {
    for entity in tilemap_query.iter_mut() {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn spawn_chunk(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    noise: Perlin,
    chunk_pos: ChunkPos,
    resource_spawn_ew: &mut EventWriter<ResourceSpawnEvent>,
) {
    let (tiles, resources) = fill_tile_chunk(noise, chunk_pos);

    let mut tile_storage = TileStorage::empty(MAP_SIZE);
    let tilemap_entity = commands.spawn_empty().id();
    let tilemap_id = TilemapId(tilemap_entity);

    resource_spawn_ew.send(ResourceSpawnEvent { resources });
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
    mut base_spawned_ew: EventWriter<BaseSpawnEvent>,
    mut resource_spawn_ew: EventWriter<ResourceSpawnEvent>,
    mut map_manager: ResMut<MapManager>,
) {
    let mut rng = rand::thread_rng();
    let seed = if map_manager.seed == 0 {
        rng.gen()
    } else {
        map_manager.seed
    };
    map_manager.seed = seed;
    let noise = Perlin::new(seed);

    //Spawn first chunk
    let chunk_pos = ChunkPos(IVec2 { x: 0, y: 0 });
    map_manager.spawned_chunks.insert(*chunk_pos);
    spawn_chunk(
        &mut commands,
        &asset_server,
        noise,
        chunk_pos,
        &mut resource_spawn_ew,
    );

    // Spawn Base
    let pos = tile_to_world_pos(
        &TilePos::new(0, 0),
        &GRID_SIZE_HEX_ROW,
        &Transform::default(),
        chunk_pos,
    );
    base_spawned_ew.send(BaseSpawnEvent { position: pos });
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
    use TerrainType::*;

    if !(0.0..=1.0).contains(&moist) || !(0.0..=1.0).contains(&temp) {
        return Mountain;
    }

    match (moist, temp) {
        (moist, _) if (0.9..=1.0).contains(&moist) => Lake,
        (_, temp) if (0.0..=0.2).contains(&temp) => Tundra,
        (moist, temp) if (0.0..=0.4).contains(&moist) && (0.5..=1.0).contains(&temp) => Desert,
        (moist, temp) if (0.5..=0.9).contains(&moist) && (0.5..=1.0).contains(&temp) => Jungle,
        (moist, temp) if (0.0..=0.4).contains(&moist) && (0.0..=0.5).contains(&temp) => Mushroom,
        (_, temp) if (0.2..=0.4).contains(&temp) => Plains,
        _ => Mountain,
    }
}

fn fill_tile_chunk(noise: Perlin, chunk_offset: ChunkPos) -> (Vec<Tile>, Vec<Resource>) {
    use TerrainType::*;

    let mut tiles: Vec<Tile> = vec![];
    let mut resources: Vec<Resource> = vec![];

    for x in 0..CHUNK_MAP_SIDE_LENGTH_X {
        for y in 0..CHUNK_MAP_SIDE_LENGTH_Y {
            let pos = TilePos::new(x, y);
            let world_pos = tile_to_world_pos(
                &pos,
                &GRID_SIZE_HEX_ROW,
                &Transform::default(),
                chunk_offset,
            );
            let alt = get_noise_value(noise, 100.5, pos, chunk_offset);
            let moist = get_noise_value(noise, 63.5, pos, chunk_offset);
            let temp = get_noise_value(noise, 33.5, pos, chunk_offset);

            let noise_val = (alt + moist + temp) / 3.0;

            // Ocean
            if noise_val < DEEP_WATER_LEVEL {
                tiles.push(Tile::new(pos, get_tile(Ocean)));
                continue;
            }

            // Shallow water
            if noise_val < SHALLOW_WATER_LEVEL {
                tiles.push(Tile::new(pos, get_tile(Lake)));
                continue;
            }

            // Beach
            if noise_val < SHORE_LEVEL {
                tiles.push(Tile::new(pos, get_tile(Beach)));
                continue;
            }

            // Other biomes
            if noise_val < MOUNTAIN_LEVEL {
                match terrain_type(moist, temp) {
                    Jungle => {
                        tiles.push(Tile::new(pos, get_tile(Jungle)));
                        resources.push(Resource::new(get_resource(Desert), world_pos));
                    }
                    Desert => {
                        tiles.push(Tile::new(pos, get_tile(Desert)));
                        resources.push(Resource::new(get_resource(Desert), world_pos));
                    }
                    Mushroom => {
                        tiles.push(Tile::new(pos, get_tile(Mushroom)));
                        resources.push(Resource::new(get_resource(Mushroom), world_pos));
                    }
                    //Tundra => {
                    //    tiles.push(Tile::new(pos, Snow));
                    //}
                    _ => {
                        if Lake == terrain_type(moist, temp) {
                            tiles.push(Tile::new(pos, get_tile(Lake)));
                        } else {
                            tiles.push(Tile::new(pos, get_tile(Plains)));
                            resources.push(Resource::new(get_resource(Plains), world_pos));

                            continue;
                        }
                    }
                }
                continue;
            }

            tiles.push(Tile::new(pos, get_tile(Mountain)));
        }
    }

    (tiles, resources)
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

pub fn spawn_nearby_chunks(
    mut commands: Commands,
    droid_query: Query<&Transform, With<Robot>>,
    asset_server: Res<AssetServer>,
    mut map_manager: ResMut<MapManager>,
    mut resource_spawn_ew: EventWriter<ResourceSpawnEvent>,
) {
    let noise = Perlin::new(map_manager.seed);
    for transform in droid_query.iter() {
        let droid_pos = Vec2::new(transform.translation.x, transform.translation.y);
        let droid_pos = droid_pos_to_chunk_pos(&droid_pos);

        for y in (droid_pos.y - 2)..(droid_pos.y + 2) {
            for x in (droid_pos.x - 2)..(droid_pos.x + 2) {
                if !map_manager.spawned_chunks.contains(&IVec2::new(x, y)) {
                    map_manager.spawned_chunks.insert(IVec2::new(x, y));
                    spawn_chunk(
                        &mut commands,
                        &asset_server,
                        noise,
                        ChunkPos(IVec2 { x, y }),
                        &mut resource_spawn_ew,
                    );
                }
            }
        }
    }
}

// TODO: Implement despawn out of range chunks if performance is an issue
fn despawn_oor_chunks() {}

fn droid_pos_to_chunk_pos(droid_pos: &Vec2) -> IVec2 {
    let droid_pos = droid_pos.as_ivec2();
    let chunk_size: IVec2 = IVec2::new(
        CHUNK_MAP_SIDE_LENGTH_X as i32,
        CHUNK_MAP_SIDE_LENGTH_Y as i32,
    );
    let tile_size: IVec2 = IVec2::new(TILE_WIDTH as i32, TILE_HEIGHT as i32);
    droid_pos / (chunk_size * tile_size)
}

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
    chunk_offset: ChunkPos,
) -> Vec2 {
    let tile_translation =
        *map_transform * tile_pos.center_in_world(grid_size, &MAP_TYPE).extend(0.0);
    let chunk_offset = Vec2::new(
        tile_pos.x as f32 + chunk_offset.x as f32 * CHUNK_MAP_SIDE_LENGTH_X as f32,
        tile_pos.y as f32 + chunk_offset.y as f32 * CHUNK_MAP_SIDE_LENGTH_Y as f32,
    );

    let world_pos = tile_translation.truncate() + chunk_offset;
    println!("world pos : {}", world_pos);
    world_pos
}

fn get_tile(biome: TerrainType) -> TileIndex {
    let rng = &mut rand::thread_rng();
    let biome_data = BIOME_DATA.get(&biome).unwrap();

    let random = rng.gen_range(0.0..1.0);
    let mut running_total = 0.0;
    for (tile, &value) in biome_data.iter() {
        running_total += value;
        if running_total >= random {
            return *tile;
        }
    }

    TileIndex::ShallowWater
}

fn get_resource(biome: TerrainType) -> ResourceType {
    let rng = &mut rand::thread_rng();
    let resource_data = RESOURCES_DATA.get(&biome).unwrap();

    let random = rng.gen_range(0.0..1.0);
    let mut running_total = 0.0;
    for (resource, &value) in resource_data.iter() {
        running_total += value;
        if running_total >= random {
            return *resource;
        }
    }

    ResourceType::None
}
