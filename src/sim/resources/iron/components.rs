use bevy::prelude::*;

#[derive(Component)]
pub struct IronParent;

#[derive(Component)]
pub struct Iron {
    pub max_health: f32,
    pub current: f32,
}
