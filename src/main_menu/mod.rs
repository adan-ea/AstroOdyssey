use bevy::prelude::*;

mod systems;

use systems::*;

use crate::AppState;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            // OnEnter State systems
            .add_systems(OnEnter(AppState::MainMenu), spawn_camera)
            // OnExit State systems
            .add_systems(OnExit(AppState::MainMenu), despawn_camera);
    }
}
