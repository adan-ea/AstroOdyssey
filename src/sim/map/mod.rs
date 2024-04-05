use bevy::prelude::*;
use bevy_ecs_tilemap::{
    map::{TilemapGridSize, TilemapSize, TilemapTileSize},
    TilemapPlugin,
};

pub mod components;
pub mod events;
mod systems;

use events::*;
use systems::*;

use crate::AppState;

use self::components::ChunkManager;

use super::SimulationState;

// Sprite sheet constants
pub const TERRAIN_SPRITE_PATH: &str = "sprites/terrain-sprite.png";
pub const TILE_HEIGHT: f32 = 140.;
pub const TILE_WIDTH: f32 = 120.;
pub const GRID_W: usize = (CHUNK_MAP_SIDE_LENGTH_X as usize) / 2;
pub const GRID_H: usize = (CHUNK_MAP_SIDE_LENGTH_Y as usize) / 2;

// If seed is set to 0, the seed will be random
pub const SEED: u32 = 4294967295;

pub const CHUNK_MAP_SIDE_LENGTH_X: u32 = 50;
pub const CHUNK_MAP_SIDE_LENGTH_Y: u32 = 50;

pub const TILE_SIZE_HEX_ROW: TilemapTileSize = TilemapTileSize {
    x: TILE_WIDTH,
    y: TILE_HEIGHT,
};

pub const GRID_SIZE_HEX_ROW: TilemapGridSize = TilemapGridSize {
    x: TILE_WIDTH,
    y: TILE_HEIGHT,
};

pub const MAP_SIZE: TilemapSize = TilemapSize {
    x: CHUNK_MAP_SIDE_LENGTH_X,
    y: CHUNK_MAP_SIDE_LENGTH_Y,
};

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app
            // Events
            .add_event::<BaseSpawnEvent>()
            // Resources
            .insert_resource(ChunkManager::default())
            // Plugins
            .add_plugins(TilemapPlugin)
            // OnEnter State systems
            .add_systems(OnEnter(AppState::Sim), setup)
            // Update State systems
            .add_systems(
                Update,
                (spawn_nearby_chunks)
                    .run_if(in_state(AppState::Sim))
                    .run_if(in_state(SimulationState::Running)),
            )
            // OnExit State systems
            .add_systems(OnExit(AppState::Sim), despawn_map);
    }
}
