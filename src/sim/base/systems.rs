use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::random;

use crate::sim::{droids::components::Droid, map::components::Tile};

use super::{Base, ExplorerSpawnTimer};

pub fn spawn_base(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    tile_query: Query<&Tile>,
) {
    let window = window_query.get_single().unwrap();

    let mut base_spawned = false;
    for entity in tile_query.iter() {
        if !entity.blocked && !base_spawned {
            if random::<f32>() < 0.1 {
                commands.spawn((
                    SpriteBundle {
                        transform: Transform::from_xyz(
                            random::<f32>() * window.width(),
                            random::<f32>() * window.height(),
                            100.0,
                        ),
                        texture: asset_server.load("tiles/tileSnow.png"),
                        ..default()
                    },
                    Base {
                        pos: entity.pos,
                        iron: 0,
                        nb_explorer_max: 10,
                    },
                    Name::new("Base"),
                ));
                base_spawned = true;
                println!("Base spawned at {:?}", entity.pos);
            }
        }
    }
}

pub fn despawn_base(mut commands: Commands, query: Query<Entity, With<Base>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

pub fn tick_explorer_spawn_timer(
    time: Res<Time>,
    mut explorer_spawn_timer: ResMut<ExplorerSpawnTimer>,
) {
    explorer_spawn_timer.time.tick(time.delta());
}

pub fn spawn_explorer_over_time(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    droid_spawn_timer: Res<ExplorerSpawnTimer>,
    droid_query: Query<Entity, With<Droid>>,
    base_query: Query<&Base>,
) {
    if droid_spawn_timer.time.finished() {
        let window = window_query.get_single().unwrap();
        if droid_query.iter().count() < base_query.single().nb_explorer_max {
            commands.spawn((
                SpriteBundle {
                    transform: Transform::from_xyz(
                        random::<f32>() * window.width(),
                        random::<f32>() * window.height(),
                        0.0,
                    ),
                    texture: asset_server.load("sprites/droids/explorer.png"),
                    ..default()
                },
                Droid {
                    energy: 100.0,
                    iron_cost: 0,
                    speed: 0.0,
                    direction: Vec2::new(0.0, 0.0),
                },
            ));
        }
    }
}
