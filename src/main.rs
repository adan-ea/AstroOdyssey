use astro_odyssey::{
    map::{gen_world, TileComponent},
    *,
};

use bevy::{prelude::*, window::close_on_esc};
use bevy_pancam::{PanCam, PanCamPlugin};

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
