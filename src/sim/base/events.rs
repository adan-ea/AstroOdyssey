use bevy::prelude::*;

#[derive(Event)]
pub struct ExplorerSpawnEvent {
    pub spawn_pos: Vec2,
}

#[derive(Event)]
pub struct CarrierSpawnEvent {
    pub spawn_pos: Vec2,
}
