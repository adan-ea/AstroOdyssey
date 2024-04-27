use bevy::prelude::*;

#[derive(Component)]
pub struct MinerParent;

#[derive(Component)]
pub struct Miner {
    pub inventory_capacity: usize,
}

impl Miner {
    pub fn new() -> Self {
        Self {
            inventory_capacity: MINER_INVENTORY_CAPACITY,
        }
    }
}
