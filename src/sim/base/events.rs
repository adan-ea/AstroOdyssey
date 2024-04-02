use bevy::prelude::*;

#[derive(Event)]
pub struct ExplorerSpawnEvent {
    pub spawn_pos: Vec2,
}
