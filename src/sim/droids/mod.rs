use bevy::prelude::*;

use crate::AppState;

use self::{components::Robot, explorer::ExplorerPlugin, healer::HealerPlugin, miner::MinerPlugin, carrier::CarrierPlugin};

pub mod components;
pub mod explorer;
pub mod healer;
pub mod miner;
pub mod carrier;
mod systems;

use systems::*;

use super::SimulationState;

const ROTATION_ANGLE: f32 = -std::f32::consts::FRAC_PI_2; // -90 degrees in radians

// Droid Idle
pub const DROID_IDLE_NEW_DESTINATION_CHANCE: u8 = 1;
pub const DROID_IDLE_ACCEPABLE_DISTANCE: f32 = 0.5;
pub const DROID_IDLE_SPEED_MULTIPLIER: f32 = 0.5;

pub struct DroidsPlugin;

impl Plugin for DroidsPlugin {
    fn build(&self, app: &mut App) {
        app
            // Plugins
            .add_plugins((ExplorerPlugin, HealerPlugin, MinerPlugin, CarrierPlugin))
            // Systems
            .add_systems(
                Update,
                (droid_idling, move_droid)
                    .run_if(in_state(AppState::Sim))
                    .run_if(in_state(SimulationState::Running)),
            )
            // OnExit State systems
            .add_systems(OnExit(AppState::Sim), despawn_droids)
            .register_type::<Robot>();
    }
}
