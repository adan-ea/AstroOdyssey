use bevy::{math::vec3, prelude::*, utils::hashbrown::HashSet, window::close_on_esc};
use bevy_pancam::{PanCam, PanCamPlugin};
use noise::{NoiseFn, Perlin};
use rand::Rng;

// Sprite sheet constants
const SPRITE_PADDING: f32 = 2.0;
const SPRITE_SCALE_FACTOR: usize = 5;
const SPRITE_SHEET_HEIGHT: usize = 9;
const SPRITE_SHEET_OFFSET: f32 = 2.0;
const SPRITE_SHEET_PATH: &str = "sprite-sheet.png";
const SPRITE_SHEET_WIDTH: usize = 8;
const TILE_HEIGHT: usize = 8;
const TILE_WIDTH: usize = 6;

// Window constants
const GRID_COLS: i32 = 200;
const GRID_ROWS: i32 = 100;
const GRID_W: usize = GRID_COLS as usize * TILE_WIDTH;
const GRID_H: usize = GRID_ROWS as usize * TILE_HEIGHT;
const BG_COLOR: (u8, u8, u8) = (181, 212, 220);

// If seed is set to 0, the seed will be random
const SEED: u32 = 0;

#[derive(Component)]
struct TileComponent;

struct Tile {
    pos: (i32, i32),
    sprite: usize,
    z_index: i32,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(PanCamPlugin)
        .insert_resource(ClearColor(Color::rgba_u8(
            BG_COLOR.0, BG_COLOR.1, BG_COLOR.2, 255,
        )))
        .insert_resource(Msaa::Off)
        .add_systems(Startup, setup)
        .add_systems(Update, handle_input)
        .add_systems(Update, close_on_esc)
        .run();
}

// Handles user input to regenerate the world when the Tab key is pressed.
fn handle_input(
    mut commands: Commands,
    keys: Res<Input<KeyCode>>,
    tiles_query: Query<Entity, With<TileComponent>>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    if !keys.just_pressed(KeyCode::Tab) {
        return;
    }

    for entity in tiles_query.iter() {
        commands.entity(entity).despawn();
    }

    gen_world(&mut commands, asset_server, &mut texture_atlases);
}

// Sets up the initial state of the application.
fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    commands
        .spawn(Camera2dBundle {
            transform: Transform::from_xyz(GRID_W as f32, GRID_H as f32, 0.0),
            ..Default::default()
        })
        .insert(PanCam::default());

    gen_world(&mut commands, asset_server, &mut texture_atlases);
}

// Generates the world by creating tiles based on noise values.
fn gen_world(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
) {
    let mut rng = rand::thread_rng();
    let mut base_spawned = false;

    let texture_handle = asset_server.load(SPRITE_SHEET_PATH);
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(TILE_WIDTH as f32, TILE_HEIGHT as f32),
        SPRITE_SHEET_WIDTH,
        SPRITE_SHEET_HEIGHT,
        Some(Vec2::splat(SPRITE_PADDING)),
        Some(Vec2::splat(SPRITE_SHEET_OFFSET)),
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

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
                tiles.push(Tile::new((x, y), 27, 5));
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
                tiles.push(Tile::new((x, y), tile, 3));
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
                tiles.push(Tile::new((x, y), tile, 3));
                continue;
            }

            // Bones
            if noise_val > 0.3 && noise_val < 0.5 && noise_val3 < 0.5 && chance > 0.98 {
                let tile = rng.gen_range(40..=43);
                tiles.push(Tile::new((x, y), tile, 1));
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
                    tiles.push(Tile::new((x, y), 19, 10));

                    // Convert grid coordinates to world coordinates
                    let (world_x, world_y) = grid_to_world(x as f32, y as f32);
                    println!(
                        "Base is at grid coordinates: ({}, {}), world coordinates: ({}, {})",
                        x, y, world_x, world_y
                    );

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

        tiles.push(Tile::new((*x, *y), tile, 0));
    }

    for tile in tiles.iter() {
        let (x, y) = tile.pos;
        let (x, y) = grid_to_world(x as f32, y as f32);
        let (x, y) = center_to_top_left(x, y);

        commands.spawn((
            SpriteSheetBundle {
                texture_atlas: texture_atlas_handle.clone(),
                sprite: TextureAtlasSprite::new(tile.sprite),
                transform: Transform::from_scale(Vec3::splat(SPRITE_SCALE_FACTOR as f32))
                    .with_translation(vec3(x, y, tile.z_index as f32)),
                ..default()
            },
            TileComponent,
        ));
    }
}

/// Determines the tile type and the number of neighboring tiles for a given position.
fn get_tile((x, y): (i32, i32), tiles: &HashSet<(i32, i32)>) -> (usize, i32) {
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
fn grid_to_world(x: f32, y: f32) -> (f32, f32) {
    (
        x * TILE_WIDTH as f32 * SPRITE_SCALE_FACTOR as f32,
        y * TILE_HEIGHT as f32 * SPRITE_SCALE_FACTOR as f32,
    )
}

// Converts center coordinates to top-left coordinates.
fn center_to_top_left(x: f32, y: f32) -> (f32, f32) {
    let x_center = x - (GRID_COLS as f32 * SPRITE_SCALE_FACTOR as f32) / 2.0;
    let y_center = ((GRID_ROWS as f32 * SPRITE_SCALE_FACTOR as f32) / 2.0) - y;

    (x_center, y_center)
}

impl Tile {
    // Creates a new Tile instance.
    fn new(pos: (i32, i32), sprite: usize, z_index: i32) -> Self {
        Self {
            pos,
            sprite,
            z_index,
        }
    }
}
