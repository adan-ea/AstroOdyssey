use bevy::prelude::*;

#[derive(Component)]
pub struct CarrierParent;

#[derive(Component)]
pub struct Carrier {
    pub exploration_radius: f32,
    pub carrier_action: CarrierAction,
}

impl Carrier {
    pub fn exploration_radius(&self) -> f32 {
        800.0
    }
    pub fn carrier_action(&self) -> CarrierAction {
        CarrierAction::Null
    }
}

#[derive(PartialEq)]
pub enum CarrierAction {
    Carrier,
    Null,
}
