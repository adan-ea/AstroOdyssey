use bevy::prelude::*;

use crate::sim::resources::components::{Resource, ResourceType};

use super::{
    components::{Iron, IronParent},
    IronSpawnEvent,
};

pub fn spawn_iron_parent(mut commands: Commands) {
    commands.spawn((
        SpatialBundle::default(),
        IronParent,
        Name::new("Iron Deposits"),
    ));
}

pub fn generate_iron(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    parent: Query<Entity, With<IronParent>>,
    mut iron_spawn_er: EventReader<IronSpawnEvent>,
) {
    let parent = parent.single();
    for iron in iron_spawn_er.read() {
        commands.entity(parent).with_children(|commands| {
            commands.spawn((
                SpriteBundle {
                    transform: Transform::from_xyz(iron.position.x, iron.position.y, 5.0),
                    texture: asset_server.load("sprites/resources/rich-iron-5.png"),
                    ..Default::default()
                },
                Iron {
                    max_health: 100.,
                    current: 100.,
                },
                Resource {
                    position: iron.position,
                    resource_type: ResourceType::Iron,
                },
            ));
        });
    }
}
