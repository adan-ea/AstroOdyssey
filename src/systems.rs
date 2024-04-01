use bevy::prelude::*;

use crate::AppState;

pub fn transition_to_game_state(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    app_state: Res<State<AppState>>,
) {
    if keyboard_input.just_pressed(KeyCode::G) {
        if **app_state != AppState::Sim {
            commands.insert_resource(NextState(Some(AppState::Sim)));
        }
    }
}

pub fn transition_to_main_menu_state(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    app_state: Res<State<AppState>>,
) {
    if keyboard_input.just_pressed(KeyCode::M) {
        if **app_state != AppState::MainMenu {
            commands.insert_resource(NextState(Some(AppState::MainMenu)));
        }
    }
}
