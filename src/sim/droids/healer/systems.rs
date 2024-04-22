use bevy::prelude::*;

use crate::sim::{
    base::events::HealerSpawnEvent,
    droids::components::{DroidState, Robot},
};

use super::{
    components::{Healer, HealerAction, HealerParent},
    HEALER_ENERGY, HEALER_EXPLORATION_RADIUS, HEALER_IRON_COST, HEALER_SPEED,
    HEALER_SPRITE_PATH,
};

pub fn spawn_healer_parent(mut commands: Commands) {
    commands.spawn((
        SpatialBundle::default(),
        HealerParent,
        Name::new("Healers"),
    ));
}

fn spawn_healer(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    parent: &mut Query<Entity, With<HealerParent>>,
    spawn_pos: Vec2,
) {
    let parent = parent.single();
    commands.entity(parent).with_children(|commands| {
        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(spawn_pos.x, spawn_pos.y, 10.0),
                texture: asset_server.load(HEALER_SPRITE_PATH),
                ..default()
            },
            Healer {
                exploration_radius: HEALER_EXPLORATION_RADIUS,
                healer_action: HealerAction::Null,
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

pub fn spawn_free_healer(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut healer_spawn_er: EventReader<HealerSpawnEvent>,
    mut parent: Query<Entity, With<HealerParent>>,
) {
    for healer_spawn in healer_spawn_er.read() {
        spawn_healer(
            &mut commands,
            &asset_server,
            &mut parent,
            healer_spawn.spawn_pos,
        );
    }
}
