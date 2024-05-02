use bevy::prelude::*;

use crate::sim::{
    base::events::MinerSpawnEvent,
    droids::{
        components::{DroidState, Energy, Robot},
        generate_random_nearby_position,
    },
};

use super::{
    components::{Miner, MinerAction, MinerParent},
    MINER_ENERGY, MINER_INVENTORY_CAPACITY, MINER_IRON_COST, MINER_SPEED, MINER_SPRITE_PATH,
};

pub fn spawn_miner_parent(mut commands: Commands) {
    commands.spawn((
        SpatialBundle::default(),
        MinerParent,
        Name::new("Miners"),
    ));
}

pub fn spawn_miner(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    parent: Query<Entity, With<MinerParent>>,
    mut miner_spawn_er: EventReader<MinerSpawnEvent>,
) {
    let parent = parent.single();
    for miner_spawn in miner_spawn_er.read() {
        let spawn_pos = generate_random_nearby_position(miner_spawn.spawn_pos);

        commands.entity(parent).with_children(|commands| {
            commands.spawn((
                SpriteBundle {
                    transform: Transform::from_xyz(spawn_pos.x, spawn_pos.y, 10.0),
                    texture: asset_server.load(MINER_SPRITE_PATH),
                    ..default()
                },
                Miner {
                    inventory_capacity: MINER_INVENTORY_CAPACITY,
                    miner_action: MinerAction::Null,
                },
                Robot {
                    energy: Energy {
                        max: MINER_ENERGY,
                        current: MINER_ENERGY,
                    },
                    speed: MINER_SPEED,
                    iron_cost: MINER_IRON_COST,
                    destination: Vec2::new(spawn_pos.x, spawn_pos.y),
                    droid_state: DroidState::Idle,
                },
                Name::new("Miner"),
            ));
        });
    }
}
