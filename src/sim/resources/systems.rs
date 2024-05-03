use bevy::prelude::*;

use super::{
    components::ResourceType, events::ResourceSpawnEvent, iron::events::IronSpawnEvent,
    tree::events::TreeSpawnEvent,
};

pub fn dispatch_resource_spawn(
    mut resource_spawn_er: EventReader<ResourceSpawnEvent>,
    mut iron_spawn_ew: EventWriter<IronSpawnEvent>,
    mut tree_spawn_ew: EventWriter<TreeSpawnEvent>,
) {
    for event in resource_spawn_er.read() {
        for resource in &event.resources {
            match resource.resource_type {
                ResourceType::Iron => iron_spawn_ew.send(IronSpawnEvent {
                    position: resource.position,
                }),
                ResourceType::PineTree => tree_spawn_ew.send(TreeSpawnEvent {
                    position: resource.position,
                }),
                _ => {
                    continue;
                }
            }
        }
    }
}
