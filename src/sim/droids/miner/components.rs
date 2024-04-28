use bevy::prelude::*;

use crate::sim::droids::miner::MINER_INVENTORY_CAPACITY;

#[derive(Component)]
pub struct MinerParent;

#[derive(Component)]
pub struct Miner {
    pub inventory_capacity: usize,
    pub miner_action: MinerAction,
}

impl Miner {
    pub fn inventory_capacity(&self) -> usize {
        MINER_INVENTORY_CAPACITY
    }
    pub fn miner_action(&self) -> MinerAction {
        MinerAction::Null
    }
}

#[derive(PartialEq)]
pub enum MinerAction {
    Mine,
    DropOff,
    Null,
}