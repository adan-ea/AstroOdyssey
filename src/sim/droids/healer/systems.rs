use bevy::prelude::*;

use crate::sim::{
    base::events::HealerSpawnEvent,
    droids::{
        components::{DroidState, Robot},
        generate_random_nearby_position,
    },
};

use super::{
    components::{Healer, HealerAction, HealerParent, EnergyConsumption},
    HEALER_ENERGY, HEALER_IRON_COST, HEALER_SPEED,
    HEALER_SPRITE_PATH, HEALER_HEALING_ENERGY,
};

pub fn spawn_healer_parent(mut commands: Commands) {
    commands.spawn((
        SpatialBundle::default(),
        HealerParent,
        Name::new("Healers"),
    ));
}

pub fn spawn_healer(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    parent: Query<Entity, With<HealerParent>>,
    mut healer_spawn_er: EventReader<HealerSpawnEvent>
) {
    let parent = parent.single();
    for healer_spawn in healer_spawn_er.read() {
        let spawn_pos = generate_random_nearby_position(healer_spawn.spawn_pos);
        

        commands.entity(parent).with_children(|commands| {
            commands.spawn((
                SpriteBundle {
                    transform: Transform::from_xyz(spawn_pos.x, spawn_pos.y, 10.0),
                    texture: asset_server.load(HEALER_SPRITE_PATH),
                    ..default()
                },
                Healer {
                    healer_action: HealerAction::Null,
                    energy_consumption: EnergyConsumption {
                        healing: HEALER_HEALING_ENERGY,
                    },
                },
                Robot {
                    energy: HEALER_ENERGY,
                    speed: HEALER_SPEED,
                    iron_cost: HEALER_IRON_COST,
                    destination: Vec2::new(spawn_pos.x, spawn_pos.y),
                    droid_state: DroidState::Idle,
                },
                Name::new("Healer"),
            ));
        });
    }
}