use bevy::prelude::*;

use crate::sim::droids::lumberjack::LUMBERJACK_INVENTORY_CAPACITY;

#[derive(Component)]
pub struct LumberjackParent;

#[derive(Component)]
pub struct Lumberjack {
    pub inventory_capacity: usize,
    pub lumberjack_action: LumberjackAction,
}

impl Lumberjack {
    pub fn inventory_capacity(&self) -> usize {
        LUMBERJACK_INVENTORY_CAPACITY
    }
    pub fn lumberjack_action(&self) -> LumberjackAction {
        LumberjackAction::Null
    }
}

#[derive(PartialEq)]
pub enum LumberjackAction {
    ChopWood,
    DropOffWood,
    BuildBridges,
    Null,
}