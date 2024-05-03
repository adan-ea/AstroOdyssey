use bevy::prelude::*;

use crate::{sim::SimulationState, AppState};

use self::systems::*;

pub mod components;
mod systems;

pub const LUMBERJACK_SPRITE_PATH: &str = "sprites/droids/lumberjack.png";
pub const LUMBERJACK_SPEED: f32 = 150.0;
pub const LUMBERJACK_ENERGY: f32 = 150.0;
pub const LUMBERJACK_INVENTORY_CAPACITY: usize = 20;
pub const LUMBERJACK_IRON_COST: u32 = 500;

pub struct LumberjackPlugin;

impl Plugin for LumberjackPlugin {
    fn build(&self, app: &mut App) {
        app
            // OnEnter State systems
            .add_systems(OnEnter(AppState::Sim), spawn_lumberjack_parent)
            // Update Systems
            .add_systems(
                Update,
                (spawn_lumberjack)
                    .run_if(in_state(AppState::Sim))
                    .run_if(in_state(SimulationState::Running)),
            );
    }
}
