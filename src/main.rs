use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

mod main_menu;
mod sim;
mod systems;

use main_menu::MainMenuPlugin;
use sim::SimPlugin;
use systems::*;

pub const BG_COLOR: (u8, u8, u8) = (181, 212, 220);

fn main() {
    App::new()
        // Bevy Plugins
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(WorldInspectorPlugin::new())
        // App State
        .add_state::<AppState>()
        // App Plugins
        .add_plugins((SimPlugin, MainMenuPlugin))
        // Resources
        .insert_resource(ClearColor(Color::rgba_u8(
            BG_COLOR.0, BG_COLOR.1, BG_COLOR.2, 255,
        )))
        .insert_resource(Msaa::Off)
        // Systems
        .add_systems(
            Update,
            (transition_to_game_state, transition_to_main_menu_state),
        )
        .run();
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    MainMenu,
    #[default]
    Sim,
    SimOver,
}
