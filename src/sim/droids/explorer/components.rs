use bevy::prelude::*;

use crate::sim::droids::components::{Droid, Robot};

#[derive(Component)]
pub struct Explorer {
    pub robot: Robot,
    pub exploration_radius: f32,
    pub explorer_action: ExplorerAction,
}

impl Droid for Explorer {
    fn energy(&self) -> f32 {
        self.robot.energy
    }

    fn speed(&self) -> f32 {
        self.robot.speed
    }

    fn iron_cost(&self) -> u32 {
        self.robot.iron_cost
    }

    fn direction(&self) -> Vec2 {
        self.robot.direction
    }
}

impl Explorer {
    pub fn exploration_radius(&self) -> f32 {
        100.0
    }
    pub fn explorer_action(&self) -> ExplorerAction {
        ExplorerAction::Null
    }
}

#[derive(PartialEq)]
pub enum ExplorerAction {
    Explore,
    Null,
}
