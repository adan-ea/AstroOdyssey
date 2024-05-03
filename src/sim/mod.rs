use bevy::prelude::*;
use bevy_pancam::PanCamPlugin;

mod base;
mod droids;
mod map;
mod resources;
mod systems;
use base::BasePlugin;
use map::MapPlugin;
use resources::ResourcesPlugin;
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
            .add_plugins((
                PanCamPlugin,
                MapPlugin,
                BasePlugin,
                DroidsPlugin,
                ResourcesPlugin,
            ))
            // OnEnter State systems
            .add_systems(OnEnter(AppState::Sim), spawn_camera)
            //Systems
            .add_systems(Update, toggle_simulation.run_if(in_state(AppState::Sim)))
            // OnExit State systems
            .add_systems(OnExit(AppState::Sim), despawn_camera);
    }
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum SimulationState {
    #[default]
    Running,
    Paused,
}
