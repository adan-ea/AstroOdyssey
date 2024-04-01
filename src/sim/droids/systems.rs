use bevy::prelude::*;
use rand::prelude::*;

use crate::sim::base::{components::Base, BASE_RADIUS};

use super::{
    components::{DroidState, Robot},
    DROID_IDLE_ACCEPABLE_DISTANCE, DROID_IDLE_NEW_DESTINATION_CHANCE, DROID_IDLE_SPEED_MULTIPLIER,
    ROTATION_ANGLE,
};

pub fn droid_idling(
    mut droid_query: Query<(&mut Transform, &mut Robot)>,
    base_query: Query<&Base>,
) {
    if let Ok(base) = base_query.get_single() {
        for (transform, mut droid) in droid_query.iter_mut() {
            if droid.droid_state != DroidState::Idle {
                return;
            }

            let droid_pos = Vec2::new(transform.translation.x, transform.translation.y);

            let distance_to_destination = (droid_pos - droid.destination).length();

            if distance_to_destination < DROID_IDLE_ACCEPABLE_DISTANCE {
                let rng = &mut rand::thread_rng();
                if rng.gen_range(0..100) < DROID_IDLE_NEW_DESTINATION_CHANCE {
                    droid.destination = generate_random_nearby_position(base.pos);
                }
            }
        }
    }
}

fn generate_random_nearby_position(base_position: Vec2) -> Vec2 {
    // Generate random offsets within the base radius
    let mut rng = rand::thread_rng();
    let dx = rng.gen_range(-BASE_RADIUS..BASE_RADIUS);
    let dy = rng.gen_range(-BASE_RADIUS..BASE_RADIUS);

    // Returns a random position near the base
    Vec2::new(base_position.x + dx, base_position.y + dy)
}

pub fn despawn_droids(mut commands: Commands, droid_query: Query<Entity, With<Robot>>) {
    for entity in droid_query.iter() {
        commands.entity(entity).despawn();
    }
}

pub fn move_droid(mut droid_query: Query<(&mut Transform, &mut Robot)>, time: Res<Time>) {
    for (mut transform, droid) in droid_query.iter_mut() {
        if droid.droid_state == DroidState::Dead || droid.droid_state == DroidState::Working {
            return;
        }

        let droid_pos = Vec2::new(transform.translation.x, transform.translation.y);

        let distance_to_destination = (droid_pos - droid.destination).length();

        if distance_to_destination > DROID_IDLE_ACCEPABLE_DISTANCE {
            let direction = (droid.destination - droid_pos).normalize();
            let direction = Vec3::new(direction.x, direction.y, 0.0);

            // Move the droid towards the destination
            transform.translation +=
                direction * (droid.speed * DROID_IDLE_SPEED_MULTIPLIER) * time.delta_seconds();

            // Rotate the droid to face the direction of movement with a custom angle
            let rotation = Quat::from_rotation_arc(Vec3::X, direction);
            let target_quaternion = rotation * Quat::from_rotation_z(ROTATION_ANGLE);

            // Smoothly interpolate between the current rotation and the target rotation
            let current_quaternion = transform.rotation;
            let interpolated_quaternion = current_quaternion.slerp(target_quaternion, 0.1);
            transform.rotation = interpolated_quaternion;
        }
    }
}
