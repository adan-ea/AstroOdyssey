use bevy::prelude::*;
use rand::Rng;

use crate::sim::droids::{
    components::Robot,
    explorer::{
        components::{Explorer, ExplorerAction},
        EXPLORER_DIRECTION, EXPLORER_ENERGY, EXPLORER_EXPLORATION_RADIUS, EXPLORER_IRON_COST,
        EXPLORER_NAME, EXPLORER_SPEED, EXPLORER_SPRITE_PATH,
    },
};

use super::{Base, ExplorerSpawnTimer};

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
    asset_server: Res<AssetServer>,
    droid_spawn_timer: Res<ExplorerSpawnTimer>,
    droid_query: Query<Entity, With<Robot>>,
    base_query: Query<&Base>,
) {
    if droid_spawn_timer.time.finished() {
        if droid_query.iter().count() < base_query.single().nb_explorer_max {
            let base_pos = base_query.single().pos;

            // Define a radius within which the explorer can spawn
            let spawn_radius = 250.0;

            // Generate random offsets within the spawn radius
            let mut rng = rand::thread_rng();
            let dx = rng.gen_range(-spawn_radius..spawn_radius);
            let dy = rng.gen_range(-spawn_radius..spawn_radius);

            // Calculate the spawn position relative to the base position
            let spawn_pos = (base_pos.0 + dx, base_pos.1 + dy);

            commands.spawn((
                SpriteBundle {
                    transform: Transform::from_xyz(spawn_pos.0, spawn_pos.1, 10.0),
                    texture: asset_server.load(EXPLORER_SPRITE_PATH),
                    ..default()
                },
                Explorer {
                    robot: Robot {
                        direction: EXPLORER_DIRECTION,
                        energy: EXPLORER_ENERGY,
                        speed: EXPLORER_SPEED,
                        iron_cost: EXPLORER_IRON_COST,
                    },
                    exploration_radius: EXPLORER_EXPLORATION_RADIUS,
                    explorer_action: ExplorerAction::Null,
                },
                Name::new(EXPLORER_NAME),
            ));
        }
    }
}
