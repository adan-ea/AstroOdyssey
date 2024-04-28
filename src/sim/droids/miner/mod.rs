use bevy::prelude::*;

use crate::{sim::SimulationState, AppState};

use self::systems::*;

pub mod components;
mod systems;

pub const MINER_SPRITE_PATH: &str = "sprites/droids/miner.png";
pub const MINER_SPEED: f32 = 90.0;
pub const MINER_ENERGY: f32 = 500.0;
pub const MINER_INVENTORY_CAPACITY: usize = 10;
pub const MINER_IRON_COST: u32 = 300;

pub struct MinerPlugin;

impl Plugin for MinerPlugin {
    fn build(&self, app: &mut App) {
        app
            // OnEnter State systems
            .add_systems(OnEnter(AppState::Sim), spawn_miner_parent)
            // Update Systems
            .add_systems(
                Update,
                (spawn_free_miner)
                    .run_if(in_state(AppState::Sim))
                    .run_if(in_state(SimulationState::Running)),
            );
    }
}
