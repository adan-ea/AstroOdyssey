use bevy::prelude::*;
use rand::Rng;

use crate::sim::{droids::explorer::components::Explorer, droids::healer::components::Healer, map::events::BaseSpawnEvent};

use super::{
    Base, ExplorerSpawnEvent, ExplorerSpawnTimer, HealerSpawnEvent, HealerSpawnTimer, BASE_MAX_EXPLORER, BASE_MAX_HEALER, BASE_RADIUS, BASE_SPRITE_PATH,
};

pub fn spawn_base(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut base_spawned_er: EventReader<BaseSpawnEvent>,
) {
    for base_spawned in base_spawned_er.read() {
        let x = base_spawned.position.x;
        let y = base_spawned.position.y;
        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(x, y, 100.0),
                texture: asset_server.load(BASE_SPRITE_PATH),
                ..default()
            },
            Base {
                pos: Vec2::new(x, y),
                iron: 0,
                nb_explorer_max: BASE_MAX_EXPLORER,
                nb_healer_max: BASE_MAX_HEALER,
            },
        ));
    }
}

pub fn despawn_base(mut commands: Commands, query: Query<Entity, With<Base>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

pub fn tick_explorer_spawn_timer(
    time: Res<Time>,
    mut explorer_spawn_timer: ResMut<ExplorerSpawnTimer>,
) {
    explorer_spawn_timer.time.tick(time.delta());
}

pub fn spawn_explorer_over_time(
    mut explorer_spawn_ew: EventWriter<ExplorerSpawnEvent>,
    explorer_spawn_timer: Res<ExplorerSpawnTimer>,
    explorer_query: Query<&Explorer>,
    base_query: Query<&Base>,
) {
    if explorer_spawn_timer.time.finished() {
        if explorer_query.iter().count() < base_query.single().nb_explorer_max {
            let base_pos = base_query.single().pos;

            // Generate random offsets within the spawn radius
            let mut rng = rand::thread_rng();
            let dx = rng.gen_range(-BASE_RADIUS..BASE_RADIUS);
            let dy = rng.gen_range(-BASE_RADIUS..BASE_RADIUS);

            // Calculate the spawn position relative to the base position
            let spawn_pos = Vec2::new(base_pos.x + dx, base_pos.y + dy);

            explorer_spawn_ew.send(ExplorerSpawnEvent { spawn_pos });
        }
    }
}

pub fn tick_healer_spawn_timer(
    time: Res<Time>,
    mut healer_spawn_timer: ResMut<HealerSpawnTimer>,
) {
    healer_spawn_timer.time.tick(time.delta());
}

pub fn spawn_healer_over_time(
    mut healer_spawn_ew: EventWriter<HealerSpawnEvent>,
    healer_spawn_timer: Res<HealerSpawnTimer>,
    healer_query: Query<&Healer>,
    base_query: Query<&Base>,
) {
    if healer_spawn_timer.time.finished() {
        if healer_query.iter().count() < base_query.single().nb_healer_max {
            let base_pos = base_query.single().pos;

            // Generate random offsets within the spawn radius
            let mut rng = rand::thread_rng();
            let dx = rng.gen_range(-BASE_RADIUS..BASE_RADIUS);
            let dy = rng.gen_range(-BASE_RADIUS..BASE_RADIUS);

            // Calculate the spawn position relative to the base position
            let spawn_pos = Vec2::new(base_pos.x + dx, base_pos.y + dy);

            healer_spawn_ew.send(HealerSpawnEvent { spawn_pos });
        }
    }
}
