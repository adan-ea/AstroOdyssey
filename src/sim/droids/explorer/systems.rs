use bevy::prelude::*;

use crate::sim::{
    base::events::ExplorerSpawnEvent,
    droids::{
        components::{DroidState, Robot},
        generate_random_nearby_position, random_name,
    },
};

use super::{
    components::{Explorer, ExplorerAction, ExplorerParent},
    EXPLORER_ENERGY, EXPLORER_EXPLORATION_RADIUS, EXPLORER_IRON_COST, EXPLORER_SPEED,
    EXPLORER_SPRITE_PATH,
};

const NAME: [&str; 17] = [
    "Etienne", "Eric", "Ethan", "Evan", "Ezra", "Ezekiel", "Eli", "Elijah", "Elias", "Emmett",
    "Emmanuel", "Eduardo", "Edward", "Edgar", "Erick", "Eduard", "Emanuel",
];

pub fn spawn_explorer_parent(mut commands: Commands) {
    commands.spawn((
        SpatialBundle::default(),
        ExplorerParent,
        Name::new("Explorers"),
    ));
}

fn spawn_explorer(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    parent: &mut Query<Entity, With<ExplorerParent>>,
    spawn_pos: Vec2,
) {
    let parent = parent.single();
    commands.entity(parent).with_children(|commands| {
        let spawn_pos = generate_random_nearby_position(spawn_pos);
        let name = random_name(NAME);
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
                name: name.clone(),
                energy: EXPLORER_ENERGY,
                speed: EXPLORER_SPEED,
                iron_cost: EXPLORER_IRON_COST,
                destination: Vec2::new(spawn_pos.x, spawn_pos.y),
                droid_state: DroidState::Idle,
            },
            Name::new(name),
        ));
    });
}

pub fn spawn_free_explorer(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut explorer_spawn_er: EventReader<ExplorerSpawnEvent>,
    mut parent: Query<Entity, With<ExplorerParent>>,
) {
    for explorer_spawn in explorer_spawn_er.read() {
        spawn_explorer(
            &mut commands,
            &asset_server,
            &mut parent,
            explorer_spawn.spawn_pos,
        );
    }
}
