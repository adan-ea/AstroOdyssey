use bevy::prelude::*;

use crate::AppState;

use self::{components::Robot, explorer::ExplorerPlugin};

pub mod components;
pub mod explorer;
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
            .add_plugins(ExplorerPlugin)
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
