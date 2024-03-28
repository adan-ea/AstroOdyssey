use bevy::math::{Quat, Vec3};
use bevy::prelude::*;
use rand::Rng;

use crate::sim::{base::components::Base, droids::components::Droid};

use super::components::{Explorer, ExplorerAction};

pub fn explorer_idling(
    mut explorer_query: Query<(&mut Transform, &Explorer)>,
    base_query: Query<&Base>,
    time: Res<Time>,
) {
    if let Ok(_base) = base_query.get_single() {
        for (mut transform, explorer) in explorer_query.iter_mut() {
            let mut rng = rand::thread_rng();
            if explorer.explorer_action() != ExplorerAction::Null {
                return;
            }

            // Check if the explorer should change direction
            let should_change_direction = rng.gen_range(0..100) < 5;
            if should_change_direction {
                // Generate a new random direction
                let new_direction =
                    Vec3::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0), 0.0).normalize();

                // Smoothly transition to the new direction over time
                let current_direction = transform.rotation * Vec3::X;
                let rotation = Quat::from_rotation_arc(current_direction, new_direction);

                // Update the explorer's transform rotation
                transform.rotation = rotation;
            } else {
                // If the droid is not changing direction, it has a chance of stopping
                if rng.gen::<f32>() < 0.2 {
                    return;
                }
            }

            let direction = transform.rotation * Vec3::X;

            transform.translation += direction * explorer.speed() * time.delta_seconds();
        }
    }
}
