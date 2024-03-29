use bevy::prelude::*;

use crate::AppState;

use self::explorer::ExplorerPlugin;

pub mod components;
pub mod explorer;
mod systems;

use systems::*;

use super::SimulationState;

pub struct DroidsPlugin;

impl Plugin for DroidsPlugin {
    fn build(&self, app: &mut App) {
        app
            // Plugins
            .add_plugins(ExplorerPlugin)
            // Systems
            .add_systems(
                Update,
                droid_idling
                    .run_if(in_state(AppState::Sim))
                    .run_if(in_state(SimulationState::Running)),
            );
    }
}
