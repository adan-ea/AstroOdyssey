use bevy::prelude::*;

#[derive(Component)]
pub struct HealerParent;

#[derive(Component)]
pub struct Healer {
    pub exploration_radius: f32,
    pub healer_action: HealerAction,
}

impl Healer {
    pub fn exploration_radius(&self) -> f32 {
        100.0
    }
    pub fn healer_action(&self) -> HealerAction {
        HealerAction::Null
    }
}

#[derive(PartialEq)]
pub enum HealerAction {
    Explore,
    Null,
}
