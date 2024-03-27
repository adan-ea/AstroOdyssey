use bevy::prelude::*;

mod base;
mod droids;
mod map;
mod systems;

use base::BasePlugin;
use map::MapPlugin;
use systems::*;

use crate::AppState;

pub struct SimPlugin;

impl Plugin for SimPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<SimulationState>()
            .add_plugins((MapPlugin, BasePlugin))
            .add_systems(Update, toggle_simulation.run_if(in_state(AppState::Sim)));
    }
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum SimulationState {
    Running,
    #[default]
    Paused,
}
