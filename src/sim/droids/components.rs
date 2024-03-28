use bevy::prelude::*;

#[derive(Component)]
pub struct Robot {
    pub energy: f32,
    pub speed: f32,
    pub iron_cost: u32,
    pub direction: Vec2,
}

pub trait Droid {
    fn energy(&self) -> f32;
    fn speed(&self) -> f32;
    fn iron_cost(&self) -> u32;
    fn direction(&self) -> Vec2;
}

impl Droid for Robot {
    fn energy(&self) -> f32 {
        self.energy
    }
    fn speed(&self) -> f32 {
        self.speed
    }
    fn iron_cost(&self) -> u32 {
        self.iron_cost
    }

    fn direction(&self) -> Vec2 {
        self.direction
    }
}

pub enum DroidState {
    Dead,
    Idle,
    Return,
    Working,
}
