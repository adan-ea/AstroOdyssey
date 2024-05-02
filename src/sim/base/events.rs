use bevy::prelude::*;

#[derive(Event)]
pub struct ExplorerSpawnEvent {
    pub spawn_pos: Vec2,
}

#[derive(Event)]
pub struct MinerSpawnEvent {
    pub spawn_pos: Vec2,
}
