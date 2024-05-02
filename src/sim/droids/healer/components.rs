use bevy::prelude::*;

#[derive(Component)]
pub struct HealerParent;

#[derive(Component)]
pub struct Healer {
    pub healer_action: HealerAction,
    pub energy_consumption: EnergyConsumption,
}

#[derive(PartialEq)]
pub struct EnergyConsumption {
    pub healing: f32,
}

impl Healer {
    pub fn healer_action(&self) -> HealerAction {
        HealerAction::Null
    }
}

#[derive(PartialEq)]
pub enum HealerAction {
    Heal,
    Null,
}
