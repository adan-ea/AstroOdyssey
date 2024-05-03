use bevy::prelude::*;

#[derive(Event)]
pub struct TreeSpawnEvent {
    pub position: Vec2,
}
