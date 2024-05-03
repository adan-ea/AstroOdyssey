use bevy::prelude::*;

#[derive(Component)]
pub struct Resource {
    pub resource_type: ResourceType,
    pub position: Vec2,
}

impl Resource {
    pub fn new(resource_type: ResourceType, position: Vec2) -> Self {
        Self {
            resource_type,
            position,
        }
    }
}

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
pub enum ResourceType {
    PineTree,
    BlueTree,
    RoundTree,
    Stone,
    Iron,
    Crystal,
    Cactus,
    None,
}
