use bevy::prelude::*;

mod components;
mod resources;
mod systems;

use components::*;
use resources::*;
use systems::*;

use crate::AppState;

use super::SimulationState;
pub struct BasePlugin;

impl Plugin for BasePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ExplorerSpawnTimer>()
            // Enter State systems
            .add_systems(OnEnter(AppState::Sim), spawn_base)
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
