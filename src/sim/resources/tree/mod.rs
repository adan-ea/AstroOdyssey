use bevy::prelude::*;

pub mod components;
pub mod events;
mod systems;

use events::*;
use systems::*;

use crate::{sim::SimulationState, AppState};

pub struct TreePlugin;

impl Plugin for TreePlugin {
    fn build(&self, app: &mut App) {
        app
            // Events
            .add_event::<TreeSpawnEvent>()
            // OnEnter State systems
            .add_systems(OnEnter(AppState::Sim), spawn_tree_parent)
            // Update State systems
            .add_systems(
                Update,
                (generate_tree)
                    .run_if(in_state(AppState::Sim))
                    .run_if(in_state(SimulationState::Running)),
            );
    }
}
