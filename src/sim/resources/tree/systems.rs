use bevy::prelude::*;

use crate::sim::resources::components::{Resource, ResourceType};

use super::{
    components::{Tree, TreeParent},
    TreeSpawnEvent,
};

pub fn spawn_tree_parent(mut commands: Commands) {
    commands.spawn((SpatialBundle::default(), TreeParent, Name::new("Forests")));
}

pub fn generate_tree(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    parent: Query<Entity, With<TreeParent>>,
    mut tree_spawn_er: EventReader<TreeSpawnEvent>,
) {
    let parent = parent.single();
    for tree in tree_spawn_er.read() {
        commands.entity(parent).with_children(|commands| {
            commands.spawn((
                SpriteBundle {
                    transform: Transform::from_xyz(tree.position.x, tree.position.y, 10.0),
                    texture: asset_server.load("sprites/resources/trees/treeRound_large.png"),
                    ..Default::default()
                },
                Tree {
                    max_health: 100.,
                    current: 100.,
                },
                Resource {
                    position: tree.position,
                    resource_type: ResourceType::PineTree,
                },
            ));
        });
    }
}
