use bevy::prelude::*;

#[derive(Component)]
pub struct Base {
    pub pos: Vec2,
    pub iron: u32,
    pub nb_explorer_max: usize,
    pub nb_healer_max: usize,
    pub nb_miner_max: usize,
    pub nb_carrier_max: usize,
    pub nb_lumberjack_max: usize,
}
