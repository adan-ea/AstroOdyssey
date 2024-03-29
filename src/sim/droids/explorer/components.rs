use bevy::prelude::*;

#[derive(Component)]
pub struct Explorer {
    pub exploration_radius: f32,
    pub explorer_action: ExplorerAction,
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
