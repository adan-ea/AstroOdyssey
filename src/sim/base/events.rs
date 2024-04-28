use bevy::prelude::*;

#[derive(Event)]
pub struct ExplorerSpawnEvent {
    pub spawn_pos: Vec2,
}

#[derive(Event)]
pub struct HealerSpawnEvent {
    pub spawn_pos: Vec2,
}