use bevy::prelude::*;

#[derive(Event)]
pub struct BaseSpawned {
    pub position: Vec2,
}

