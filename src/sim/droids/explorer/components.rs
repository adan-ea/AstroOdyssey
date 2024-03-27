use bevy::prelude::*;

use crate::sim::droids::components::Droid;

#[derive(Component)]
pub struct Explorer {
    pub droid: Droid,
    pub exploration_radius: f32,
}
