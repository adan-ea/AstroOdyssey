use bevy::prelude::*;

use crate::sim::{
    base::events::CarrierSpawnEvent,
    droids::{
        components::{DroidState, Robot},
        generate_random_nearby_position,
    },
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

pub fn spawn_carrier(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    parent: Query<Entity, With<CarrierParent>>,
    mut carrier_spawn_er: EventReader<CarrierSpawnEvent>,
) {
    let parent = parent.single();
    for carrier_spawn in carrier_spawn_er.read() {
        let spawn_pos = generate_random_nearby_position(carrier_spawn.spawn_pos);

        commands.entity(parent).with_children(|commands| {
            commands.spawn((
                SpriteBundle {
                    transform: Transform::from_xyz(spawn_pos.x, spawn_pos.y, 10.0),
                    texture: asset_server.load(CARRIER_SPRITE_PATH),
                    ..default()
                },
                Carrier {
                    inventory_capacity: CARRIER_INVENTORY_CAPACITY,
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
}
