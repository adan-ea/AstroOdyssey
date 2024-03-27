use bevy::prelude::*;

mod map;
mod systems;

use map::MapPlugin;
use systems::*;

use crate::AppState;

pub struct SimPlugin;

impl Plugin for SimPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<SimulationState>()
            .add_plugins(MapPlugin)
            .add_systems(Update, toggle_simulation.run_if(in_state(AppState::Sim)));
    }
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum SimulationState {
    Running,
    #[default]
    Paused,
}
