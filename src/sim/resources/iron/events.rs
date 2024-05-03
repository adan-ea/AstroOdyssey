use bevy::prelude::*;

#[derive(Event)]
pub struct IronSpawnEvent {
    pub position: Vec2,
}
