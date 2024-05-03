use bevy::prelude::*;

use crate::sim::{
    base::events::LumberjackSpawnEvent,
    droids::{
        components::{DroidState, Robot},
        generate_random_nearby_position,
    },
};

use super::{
    components::{Lumberjack, LumberjackAction, LumberjackParent},
    LUMBERJACK_ENERGY, LUMBERJACK_INVENTORY_CAPACITY,LUMBERJACK_IRON_COST, LUMBERJACK_SPEED, LUMBERJACK_SPRITE_PATH,
};

pub fn spawn_lumberjack_parent(mut commands: Commands) {
    commands.spawn((
        SpatialBundle::default(),
        LumberjackParent,
        Name::new("Lumberjacks"),
    ));
}

pub fn spawn_lumberjack(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    parent: Query<Entity, With<LumberjackParent>>,
    mut lumberjack_spawn_er: EventReader<LumberjackSpawnEvent>,
) {
    let parent = parent.single();
    for lumberjack_spawn in lumberjack_spawn_er.read() {
        let spawn_pos = generate_random_nearby_position(lumberjack_spawn.spawn_pos);

        commands.entity(parent).with_children(|commands| {
            commands.spawn((
                SpriteBundle {
                    transform: Transform::from_xyz(spawn_pos.x, spawn_pos.y, 10.0),
                    texture: asset_server.load(LUMBERJACK_SPRITE_PATH),
                    ..default()
                },
                Lumberjack {
                    inventory_capacity: LUMBERJACK_INVENTORY_CAPACITY,
                    lumberjack_action: LumberjackAction::Null,
                },
                Robot {
                    energy: LUMBERJACK_ENERGY,
                    speed: LUMBERJACK_SPEED,
                    iron_cost: LUMBERJACK_IRON_COST,
                    destination: Vec2::new(spawn_pos.x, spawn_pos.y),
                    droid_state: DroidState::Idle,
                },
                Name::new("Lumberjack"),
            ));
        });
    }
}
