use bevy::{prelude::*, window::close_on_esc};
use noise::{NoiseFn, Perlin, Seedable};
use rand::Rng;

// Sprite sheet constants
const SPRITE_SHEET_PATH: &str = "sprite-sheet.png";
const SPRITE_SCALE_FACTOR: usize = 5;
const TILE_WIDTH: usize = 8;
const TILE_HEIGHT: usize = 8;

//Window constants
pub const GRID_COLS: usize = 100;
pub const GRID_ROWS: usize = 100;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_systems(Startup, setup)
        .add_systems(Update, close_on_esc)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let mut rng = rand::thread_rng();
    let perlin = Perlin::new(rng.gen());

    commands.spawn(Camera2dBundle::default());

    let texture_handle = asset_server.load(SPRITE_SHEET_PATH);
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(TILE_WIDTH as f32, TILE_HEIGHT as f32),
        7,
        1,
        None,
        None,
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands.spawn((SpriteSheetBundle {
        texture_atlas: texture_atlas_handle,
        sprite: TextureAtlasSprite::new(2),
        transform: Transform::from_scale(Vec3::splat(SPRITE_SCALE_FACTOR as f32)),
        ..default()
    },));

    for x in 0..GRID_COLS {
        for y in 0..GRID_ROWS {
            let val = perlin.get([x as f64, y as f64]);
            println!("{}", val);
        }
    }
}
