use bevy::prelude::*;

use super::SimulationState;

pub fn toggle_simulation(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
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
}
