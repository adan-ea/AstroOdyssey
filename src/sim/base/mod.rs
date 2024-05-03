use bevy::prelude::*;

pub mod components;
pub mod events;
mod resources;
mod systems;

use components::*;
use events::*;
use resources::*;
use systems::*;

use crate::AppState;

use super::SimulationState;

pub const BASE_SPRITE_PATH: &str = "sprites/tiles/base.png";
pub const BASE_RADIUS: f32 = 250.0;
pub const BASE_MAX_EXPLORER: usize = 10;
pub const BASE_MAX_HEALER: usize = 10;
pub const BASE_MAX_MINER: usize = 10;
pub const BASE_MAX_CARRIER: usize = 10;
pub const BASE_MAX_LUMBERJACK: usize = 5;

pub struct BasePlugin;

impl Plugin for BasePlugin {
    fn build(&self, app: &mut App) {
        app
            // Resources
            .init_resource::<ExplorerSpawnTimer>()
            // Events
            .add_event::<ExplorerSpawnEvent>()
            .add_event::<HealerSpawnEvent>()
            .add_event::<MinerSpawnEvent>()
            .add_event::<CarrierSpawnEvent>()
            .add_event::<LumberjackSpawnEvent>()
            // Systems
            .add_systems(
                Update,
                (
                    tick_explorer_spawn_timer,
                    spawn_explorer_over_time,
                    spawn_base,
                )
                    .run_if(in_state(AppState::Sim))
                    .run_if(in_state(SimulationState::Running)),
            )
            // Exit State systems
            .add_systems(OnExit(AppState::Sim), despawn_base);
    }
}
