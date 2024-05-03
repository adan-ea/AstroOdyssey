use bevy::prelude::*;

pub mod components;
pub mod events;
mod systems;

use events::*;
use systems::*;

use crate::{sim::SimulationState, AppState};

pub struct IronPlugin;

impl Plugin for IronPlugin {
    fn build(&self, app: &mut App) {
        app
            // Events
            .add_event::<IronSpawnEvent>()
            // OnEnter State systems
            .add_systems(OnEnter(AppState::Sim), spawn_iron_parent)
            // Update State systems
            .add_systems(
                Update,
                (generate_iron)
                    .run_if(in_state(AppState::Sim))
                    .run_if(in_state(SimulationState::Running)),
            );
    }
}
