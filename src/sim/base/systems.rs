use bevy::prelude::*;

use crate::sim::{droids::explorer::components::Explorer, map::events::BaseSpawnEvent};

use super::{
    Base, CarrierSpawnEvent, ExplorerSpawnEvent, ExplorerSpawnTimer, HealerSpawnEvent,
    LumberjackSpawnEvent, MinerSpawnEvent, BASE_MAX_CARRIER, BASE_MAX_EXPLORER, BASE_MAX_HEALER,
    BASE_MAX_LUMBERJACK, BASE_MAX_MINER, BASE_SPRITE_PATH,
};

pub fn spawn_base(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut base_spawned_er: EventReader<BaseSpawnEvent>,
    mut healer_spawn_ew: EventWriter<HealerSpawnEvent>,
    mut miner_spawn_ew: EventWriter<MinerSpawnEvent>,
    mut carrier_spawn_ew: EventWriter<CarrierSpawnEvent>,
    mut lumberjack_spawn_ew: EventWriter<LumberjackSpawnEvent>,
    mut explorer_spawn_ew: EventWriter<ExplorerSpawnEvent>,
) {
    for base_spawned in base_spawned_er.read() {
        let x = base_spawned.position.x;
        let y = base_spawned.position.y;
        let spawn_pos = Vec2::new(x, y);

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(x, y, 100.0),
                texture: asset_server.load(BASE_SPRITE_PATH),
                ..default()
            },
            Base {
                pos: spawn_pos,
                iron: 0,
                nb_explorer_max: BASE_MAX_EXPLORER,
                nb_healer_max: BASE_MAX_HEALER,
                nb_miner_max: BASE_MAX_MINER,
                nb_carrier_max: BASE_MAX_CARRIER,
                nb_lumberjack_max: BASE_MAX_LUMBERJACK,
            },
        ));
        healer_spawn_ew.send(HealerSpawnEvent { spawn_pos });
        miner_spawn_ew.send(MinerSpawnEvent { spawn_pos });
        carrier_spawn_ew.send(CarrierSpawnEvent { spawn_pos });
        lumberjack_spawn_ew.send(LumberjackSpawnEvent { spawn_pos });
        explorer_spawn_ew.send(ExplorerSpawnEvent { spawn_pos });
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
            let spawn_pos = base_query.single().pos;

            explorer_spawn_ew.send(ExplorerSpawnEvent { spawn_pos });
        }
    }
}
