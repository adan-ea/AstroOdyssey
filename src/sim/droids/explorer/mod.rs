use bevy::prelude::*;

use crate::{sim::SimulationState, AppState};

use self::systems::*;

pub mod components;
mod systems;

pub const EXPLORER_SPRITE_PATH: &str = "sprites/droids/explorer.png";
pub const EXPLORER_SPEED: f32 = 200.0;
pub const EXPLORER_ENERGY: f32 = 100.0;
pub const EXPLORER_EXPLORATION_RADIUS: f32 = 100.0;
pub const EXPLORER_IRON_COST: u32 = 100;
pub const EXPLORER_DIRECTION: Vec2 = Vec2::new(1.0, 1.0);

pub struct ExplorerPlugin;

impl Plugin for ExplorerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (spawn_explorer)
                .run_if(in_state(AppState::Sim))
                .run_if(in_state(SimulationState::Running)),
        );
    }
}
