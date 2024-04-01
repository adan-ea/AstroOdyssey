use bevy::prelude::*;

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct Robot {
    pub energy: f32,
    pub speed: f32,
    pub iron_cost: u32,
    pub destination: Vec2,
    pub droid_state: DroidState,
}

pub trait Droid {
    fn energy(&self) -> f32;
    fn speed(&self) -> f32;
    fn iron_cost(&self) -> u32;
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

}

#[derive(PartialEq, Reflect)]
pub enum DroidState {
    Dead,
    Idle,
    Return,
    Working,
}

impl Default for DroidState {
    fn default() -> Self {
        DroidState::Idle
    }
}
