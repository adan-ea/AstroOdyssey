use bevy::prelude::*;

use crate::AppState;

use super::SimulationState;

pub fn toggle_simulation(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    simulation_state: ResMut<State<SimulationState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        if **simulation_state == SimulationState::Running {
            commands.insert_resource(NextState(Some(SimulationState::Paused)));
            println!("Paused");
        }
        if **simulation_state == SimulationState::Paused {
            commands.insert_resource(NextState(Some(SimulationState::Running)));
            println!("Running :]");
        }
    }
    
    if keyboard_input.just_pressed(KeyCode::KeyR) {
        commands.insert_resource(NextState(Some(SimulationState::Paused)));
        commands.insert_resource(NextState(Some(AppState::SimOver)));
        commands.insert_resource(NextState(Some(AppState::Sim)));
        commands.insert_resource(NextState(Some(SimulationState::Running)));
        println!("Reset");
    }
}
