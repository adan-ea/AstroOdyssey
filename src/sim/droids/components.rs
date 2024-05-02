use bevy::prelude::*;

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct Robot {
    pub energy: Energy,
    pub speed: f32,
    pub iron_cost: u32,
    pub destination: Vec2,
    pub droid_state: DroidState,
}

#[derive(Component, Reflect)]
pub struct Energy {
    pub max: f32,
    pub current: f32,
}

impl Default for Energy {
    fn default() -> Self {
        Self {
            max: 100.0,
            current: 100.0,
        }
    }
}

pub trait Droid {
    fn speed(&self) -> f32;
    fn iron_cost(&self) -> u32;
}

impl Droid for Robot {
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
