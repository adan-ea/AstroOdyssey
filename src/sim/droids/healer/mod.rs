use bevy::prelude::*;

use crate::{sim::SimulationState, AppState};

use self::systems::*;

pub mod components;
mod systems;

pub const HEALER_SPRITE_PATH: &str = "sprites/droids/healer.png";
pub const HEALER_SPEED: f32 = 130.0;
pub const HEALER_ENERGY: f32 = 500.0;
pub const HEALER_IRON_COST: u32 = 300;

pub struct HealerPlugin;

impl Plugin for HealerPlugin {
    fn build(&self, app: &mut App) {
        app
            // OnEnter State systems
            .add_systems(OnEnter(AppState::Sim), spawn_healer_parent)
            // Update Systems
            .add_systems(
                Update,
                (spawn_free_healer)
                    .run_if(in_state(AppState::Sim))
                    .run_if(in_state(SimulationState::Running)),
            );
    }
}
