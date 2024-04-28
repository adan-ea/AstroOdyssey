use bevy::prelude::*;

use crate::sim::{
    base::events::MinerSpawnEvent,
    droids::components::{DroidState, Robot},
};

use super::{
    components::{Miner, MinerAction,MinerParent},
    MINER_ENERGY, MINER_INVENTORY_CAPACITY, MINER_IRON_COST, MINER_SPEED,
    MINER_SPRITE_PATH,
};

pub fn spawn_miner_parent(mut commands: Commands) {
    commands.spawn((
        SpatialBundle::default(),
        MinerParent,
        Name::new("Miners"),
    ));
}

fn spawn_miner(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    parent: &mut Query<Entity, With<MinerParent>>,
    spawn_pos: Vec2,
) {
    let parent = parent.single();
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
                energy: MINER_ENERGY,
                speed: MINER_SPEED,
                iron_cost: MINER_IRON_COST,
                destination: Vec2::new(spawn_pos.x, spawn_pos.y),
                droid_state: DroidState::Idle,
            },
            Name::new("Miner"),
        ));
    });
}

pub fn spawn_free_miner(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut miner_spawn_er: EventReader<MinerSpawnEvent>,
    mut parent: Query<Entity, With<MinerParent>>,
) {
    for miner_spawn in miner_spawn_er.read() {
        spawn_miner(
            &mut commands,
            &asset_server,
            &mut parent,
            miner_spawn.spawn_pos,
        );
    }
}
