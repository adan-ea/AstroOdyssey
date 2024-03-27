use bevy::prelude::*;

#[derive(Component)]
pub struct Droid {
    pub energy: f32,
    pub iron_cost: u32,
    pub speed: f32,
    pub direction: Vec2,
}