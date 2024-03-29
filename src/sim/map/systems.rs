use bevy::{prelude::*, utils::hashbrown::HashSet};
use bevy_pancam::PanCam;
use noise::{NoiseFn, Perlin};
use rand::Rng;

use crate::sim::base::{components::Base, BASE_MAX_EXPLORER};

use super::{
    components::Tile, GRID_COLS, GRID_H, GRID_ROWS, GRID_W, SEED, SPRITE_PADDING,
    SPRITE_SCALE_FACTOR, SPRITE_SHEET_HEIGHT, SPRITE_SHEET_OFFSET, SPRITE_SHEET_PATH,
    SPRITE_SHEET_WIDTH, TILE_HEIGHT, TILE_WIDTH,
};

// Despawn all tiles when exiting the simulation state.
pub fn despawn_map(mut commands: Commands, tiles_query: Query<Entity, With<Tile>>) {
    for entity in tiles_query.iter() {
        commands.entity(entity).despawn();
    }
}

// Sets up the initial state of the application.
pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    commands
        .spawn(Camera2dBundle {
            transform: Transform::from_xyz(GRID_W as f32, GRID_H as f32, 0.0),
            ..Default::default()
        })
        .insert(PanCam::default());

    gen_world(&mut commands, asset_server, texture_atlases);
}

// Generates the world by creating tiles based on noise values.
pub fn gen_world(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    let mut rng = rand::thread_rng();
    let mut base_spawned = false;

    let texture_handle = asset_server.load(SPRITE_SHEET_PATH);
    let layout = TextureAtlasLayout::from_grid(
        Vec2::new(TILE_WIDTH as f32, TILE_HEIGHT as f32),
        SPRITE_SHEET_WIDTH,
        SPRITE_SHEET_HEIGHT,
        Some(Vec2::splat(SPRITE_PADDING)),
        Some(Vec2::splat(SPRITE_SHEET_OFFSET)),
    );
    let texture_atlas_handle = texture_atlases.add(layout);

    let seed = if SEED == 0 { rng.gen() } else { SEED };
    let noise = Perlin::new(seed);

    let mut tiles = Vec::new();
    let mut occupied = HashSet::new();
    for x in 0..GRID_COLS {
        for y in 0..GRID_ROWS {
            let noise_val1 = noise.get([x as f64 / 100.5, y as f64 / 100.5]);
            let noise_val2 = noise.get([x as f64 / 53.5, y as f64 / 53.5]);
            let noise_val3 = noise.get([x as f64 / 43.5, y as f64 / 43.5]);
            let noise_val4 = noise.get([x as f64 / 23.5, y as f64 / 23.5]);
            let noise_val = (noise_val1 + noise_val2 + noise_val3 + noise_val4) / 4.0;
            let chance = rng.gen_range(0.0..1.0);

            // Ground
            if noise_val > 0.1 {
                occupied.insert((x, y));
            } else {
                continue;
            }

            // Too close to shore
            if noise_val < 0.15 {
                continue;
            }

            // Dense Forest
            if (noise_val > 0.5 || noise_val3 > 0.98) && chance > 0.2 {
                tiles.push(Tile::new((x, y), 27, 5, true));
                continue;
            }
            // Patch Forest
            if noise_val3 > 0.5 && noise_val < 0.5 && chance > 0.4 {
                let chance2 = rng.gen_range(0.0..1.0);
                let tile = if chance2 > 0.7 {
                    rng.gen_range(24..=26)
                } else {
                    rng.gen_range(24..=25)
                };
                tiles.push(Tile::new((x, y), tile, 3, true));
                continue;
            }
            // Sparse Forest
            if noise_val4 > 0.4 && noise_val < 0.5 && noise_val3 < 0.5 && chance > 0.9 {
                let chance = rng.gen_range(0.0..1.0);
                let tile = if chance > 0.78 {
                    rng.gen_range(28..=29)
                } else {
                    rng.gen_range(24..=25)
                };
                tiles.push(Tile::new((x, y), tile, 3, false));
                continue;
            }

            // Bones
            if noise_val > 0.3 && noise_val < 0.5 && noise_val3 < 0.5 && chance > 0.98 {
                let tile = rng.gen_range(40..=43);
                tiles.push(Tile::new((x, y), tile, 1, false));
                continue;
            }

            // Settlement
            if !base_spawned
                && noise_val > 0.1
                && noise_val < 0.3
                && noise_val3 < 0.4
                && chance > 0.8
            {
                let chance2 = rng.gen_range(0.0..1.0);

                if chance2 > 0.98 {
                    let (x, y) = grid_to_world(x as f32, y as f32);
                    let (x, y) = center_to_top_left(x, y);

                    commands.spawn((
                        SpriteBundle {
                            transform: Transform::from_xyz(x, y, 100.0),
                            texture: asset_server.load("tiles/tileSnow.png"),
                            ..default()
                        },
                        Base {
                            pos: Vec2::new(x, y),
                            iron: 0,
                            nb_explorer_max: BASE_MAX_EXPLORER,
                        },
                        Name::new("Base"),
                    ));
                    base_spawned = true;
                }

                continue;
            }
        }
    }

    for (x, y) in occupied.iter() {
        let (tile, nei_count) = get_tile((*x, *y), &occupied);

        if nei_count == 1 {
            continue;
        }

        tiles.push(Tile::new((*x, *y), tile, 0, false));
    }

    for tile in tiles.iter() {
        let (x, y) = tile.pos;
        let (x, y) = grid_to_world(x as f32, y as f32);
        let (x, y) = center_to_top_left(x, y);

        // Spawn tiles using the new SpriteSheetBundle initialization
        commands.spawn((
            SpriteSheetBundle {
                atlas: TextureAtlas {
                    layout: texture_atlas_handle.clone(),
                    index: tile.sprite,
                },
                texture: texture_handle.clone(),
                transform: Transform::from_scale(Vec3::splat(SPRITE_SCALE_FACTOR as f32))
                    .with_translation(Vec3::new(x, y, tile.z_index as f32)),
                ..Default::default()
            },
            Tile {
                pos: tile.pos,
                sprite: tile.sprite,
                z_index: tile.z_index,
                blocked: tile.blocked,
                known: tile.known,
            },
            Name::new("Tile"),
        ));
    }
}

// Determines the tile type and the number of neighboring tiles for a given position.
pub fn get_tile((x, y): (i32, i32), tiles: &HashSet<(i32, i32)>) -> (usize, i32) {
    let (x, y) = (x, y);
    let nei_options = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    let mut nei = [1; 4];
    let mut nei_count = 0;

    for (idx, (i, j)) in nei_options.iter().enumerate() {
        if tiles.contains(&(x + i, y + j)) {
            nei_count += 1;
            continue;
        }
        nei[idx] = 0;
    }

    let tile = match nei {
        [0, 1, 1, 0] => 3,
        [1, 0, 1, 0] => 4,
        [0, 1, 0, 1] => 1,
        [1, 0, 0, 1] => 2,
        _ => 0,
    };

    (tile, nei_count)
}

// Converts grid coordinates to world coordinates.
pub fn grid_to_world(x: f32, y: f32) -> (f32, f32) {
    (
        x * TILE_WIDTH as f32 * SPRITE_SCALE_FACTOR as f32,
        y * TILE_HEIGHT as f32 * SPRITE_SCALE_FACTOR as f32,
    )
}

// Converts center coordinates to top-left coordinates.
pub fn center_to_top_left(x: f32, y: f32) -> (f32, f32) {
    let x_center = x - (GRID_W as f32 * SPRITE_SCALE_FACTOR as f32) / 2.0;
    let y_center = (GRID_H as f32 * SPRITE_SCALE_FACTOR as f32) / 2.0 - y;
    (x_center, y_center)
}
