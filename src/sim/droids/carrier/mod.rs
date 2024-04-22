use bevy::prelude::*;

use crate::{sim::SimulationState, AppState};

use self::systems::*;

pub mod components;
mod systems;

pub const CARRIER_SPRITE_PATH: &str = "sprites/droids/carrier.png";
pub const CARRIER_SPEED: f32 = 200.0;
pub const CARRIER_ENERGY: f32 = 800.0;
pub const CARRIER_EXPLORATION_RADIUS: f32 = 800.0;
pub const CARRIER_IRON_COST: u32 = 100;

pub struct CarrierPlugin;

impl Plugin for CarrierPlugin {
    fn build(&self, app: &mut App) {
        app
            // OnEnter State systems
            .add_systems(OnEnter(AppState::Sim), spawn_carrier_parent)
            // Update Systems
            .add_systems(
                Update,
                (spawn_free_carrier)
                    .run_if(in_state(AppState::Sim))
                    .run_if(in_state(SimulationState::Running)),
            );
    }
}
