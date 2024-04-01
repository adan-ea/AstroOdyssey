use bevy::prelude::*;

use crate::sim::{
    base::events::ExplorerSpawnEvent,
    droids::components::{DroidState, Robot},
};

use super::{
    components::{Explorer, ExplorerAction},
    EXPLORER_DIRECTION, EXPLORER_ENERGY, EXPLORER_EXPLORATION_RADIUS, EXPLORER_IRON_COST,
    EXPLORER_SPEED, EXPLORER_SPRITE_PATH,
};

pub fn spawn_explorer(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut explorer_spawn_er: EventReader<ExplorerSpawnEvent>,
) {
    for explorer_spawn in explorer_spawn_er.read() {
        let spawn_pos = explorer_spawn.spawn_pos;
        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(spawn_pos.x, spawn_pos.y, 10.0),
                texture: asset_server.load(EXPLORER_SPRITE_PATH),
                ..default()
            },
            Explorer {
                exploration_radius: EXPLORER_EXPLORATION_RADIUS,
                explorer_action: ExplorerAction::Null,
            },
            Robot {
                direction: EXPLORER_DIRECTION,
                energy: EXPLORER_ENERGY,
                speed: EXPLORER_SPEED,
                iron_cost: EXPLORER_IRON_COST,
                destination: Vec2::new(spawn_pos.x, spawn_pos.y),
                droid_state: DroidState::Idle,
            },
            Name::new("Explorer")
        ));
    }
}
