use bevy::prelude::*;

use crate::sim::droids::carrier::CARRIER_INVENTORY_CAPACITY;

#[derive(Component)]
pub struct CarrierParent;

#[derive(Component)]
pub struct Carrier {
    pub inventory_capacity: usize,
    pub carrier_action: CarrierAction,
}

impl Carrier {
    pub fn inventory_capacity(&self) -> usize {
        CARRIER_INVENTORY_CAPACITY
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
