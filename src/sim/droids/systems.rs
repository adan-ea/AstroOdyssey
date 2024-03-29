use bevy::prelude::*;
use rand::prelude::*;

use crate::sim::base::{components::Base, BASE_RADIUS};

use super::components::{DroidState, Robot};

pub fn droid_idling(
    mut droid_query: Query<(&mut Transform, &mut Robot)>, // Make the Robot component mutable
    base_query: Query<&Base>,
    time: Res<Time>,
) {
    if let Ok(base) = base_query.get_single() {
        for (mut transform, mut droid) in droid_query.iter_mut() {
            if droid.droid_state != DroidState::Idle {
                return;
            }

            let droid_pos = Vec2::new(transform.translation.x, transform.translation.y);
            let distance_to_destination = (droid_pos - droid.destination).length();
            if distance_to_destination > 0.1 {
                let direction = (droid.destination - droid_pos).normalize();
                let direction = Vec3::new(direction.x, direction.y, 0.0);
                transform.translation += direction * (droid.speed / 2.0) * time.delta_seconds();
                let rotation = Quat::from_rotation_arc(Vec3::X, direction);
                transform.rotation = rotation;
            } else {
                droid.destination = generate_random_nearby_position(base.pos);
            }
        }
    }
}

fn generate_random_nearby_position(base_position: Vec2) -> Vec2 {
    // Generate random offsets within the base radius
    let mut rng = rand::thread_rng();
    let dx = rng.gen_range(-BASE_RADIUS..BASE_RADIUS);
    let dy = rng.gen_range(-BASE_RADIUS..BASE_RADIUS);

    // Calculate the spawn position relative to the base position
    Vec2::new(base_position.x + dx, base_position.y + dy)
}
