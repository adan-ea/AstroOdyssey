use bevy::prelude::*;

use crate::sim::{
    base::events::CarrierSpawnEvent,
    droids::components::{DroidState, Robot},
};

use super::{
    components::{Carrier, CarrierAction, CarrierParent},
    CARRIER_ENERGY, CARRIER_INVENTORY_CAPACITY, CARRIER_IRON_COST, CARRIER_SPEED, CARRIER_SPRITE_PATH,
};

pub fn spawn_carrier_parent(mut commands: Commands) {
    commands.spawn((
        SpatialBundle::default(),
        CarrierParent,
        Name::new("Carriers"),
    ));
}

fn spawn_carrier(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    parent: &mut Query<Entity, With<CarrierParent>>,
    spawn_pos: Vec2,
) {
    let parent = parent.single();
    commands.entity(parent).with_children(|commands| {
        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(spawn_pos.x, spawn_pos.y, 10.0),
                texture: asset_server.load(CARRIER_SPRITE_PATH),
                ..default()
            },
            Carrier {
                carrier_radius: CARRIER_INVENTORY_CAPACITY,
                carrier_action: CarrierAction::Null,
            },
            Robot {
                energy: CARRIER_ENERGY,
                speed: CARRIER_SPEED,
                iron_cost: CARRIER_IRON_COST,
                destination: Vec2::new(spawn_pos.x, spawn_pos.y),
                droid_state: DroidState::Idle,
            },
            Name::new("Carrier"),
        ));
    });
}

pub fn spawn_carrier(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut carrier_spawn_es: EventReader<CarrierSpawnEvent>,
    mut parent: Query<Entity, With<CarrierParent>>,
) {
    for carrier_spawn in carrier_spawn_es.read() {
        spawn_carrier(
            &mut commands,
            &asset_server,
            &mut parent,
            carrier_spawn.spawn_pos,
        );
    }
}
