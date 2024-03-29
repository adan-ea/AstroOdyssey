use bevy::prelude::*;

pub mod components;
mod resources;
mod systems;

use components::*;
use resources::*;
use systems::*;

use crate::AppState;

use super::SimulationState;

pub const BASE_RADIUS: f32 = 250.0;
pub const BASE_MAX_EXPLORER: usize = 10;

pub struct BasePlugin;

impl Plugin for BasePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ExplorerSpawnTimer>()
            // Systems
            .add_systems(
                Update,
                (tick_explorer_spawn_timer, spawn_explorer_over_time)
                    .run_if(in_state(AppState::Sim))
                    .run_if(in_state(SimulationState::Running)),
            )
            // Exit State systems
            .add_systems(OnExit(AppState::Sim), despawn_base);
    }
}
