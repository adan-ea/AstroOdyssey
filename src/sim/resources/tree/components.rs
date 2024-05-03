use bevy::prelude::*;

#[derive(Component)]
pub struct TreeParent;

#[derive(Component)]
pub struct Tree {
    pub max_health: f32,
    pub current: f32,
}
