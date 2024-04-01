use bevy::prelude::*;

mod base;
mod droids;
mod map;
mod systems;

use base::BasePlugin;
use map::MapPlugin;
use systems::*;

use crate::AppState;

use self::droids::DroidsPlugin;

pub struct SimPlugin;

impl Plugin for SimPlugin {
    fn build(&self, app: &mut App) {
        app
            // States
            .add_state::<SimulationState>()
            // App Plugins
            .add_plugins((MapPlugin, BasePlugin, DroidsPlugin))
            // OnEnter State systems
            .add_systems(OnEnter(AppState::Sim), spawn_camera)
            //Systems
            .add_systems(Update, toggle_simulation.run_if(in_state(AppState::Sim)));
    }
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum SimulationState {
    #[default]
    Running,
    Paused,
}
