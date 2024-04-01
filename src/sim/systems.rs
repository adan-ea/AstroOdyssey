use bevy::prelude::*;
use bevy_pancam::PanCam;

use crate::AppState;

use super::{map::{GRID_H, GRID_W}, SimulationState};

pub fn toggle_simulation(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    simulation_state: ResMut<State<SimulationState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        if **simulation_state == SimulationState::Running {
            commands.insert_resource(NextState(Some(SimulationState::Paused)));
        }
        if **simulation_state == SimulationState::Paused {
            commands.insert_resource(NextState(Some(SimulationState::Running)));
        }
    }
    // TODO: Make this work somehow :')
    if keyboard_input.just_pressed(KeyCode::R) {
        commands.insert_resource(NextState(Some(SimulationState::Paused)));
        commands.insert_resource(NextState(Some(AppState::SimOver)));
        commands.insert_resource(NextState(Some(AppState::Sim)));
        commands.insert_resource(NextState(Some(SimulationState::Running)));
        println!("Reset");
    }
}

pub fn spawn_camera(mut commands: Commands) {
    commands
    .spawn(Camera2dBundle {
        transform: Transform::from_xyz(GRID_W as f32, GRID_H as f32, 0.0),
        ..Default::default()
    })
    .insert(PanCam::default());
}