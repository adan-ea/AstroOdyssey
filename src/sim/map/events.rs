use bevy::prelude::*;

#[derive(Event)]
pub struct BaseSpawnEvent {
    pub position: Vec2,
}

