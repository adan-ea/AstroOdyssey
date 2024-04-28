use bevy::prelude::*;

const EXPLORER_SPAWN_INTERVAL: f32 = 5.0;
const MINER_SPAWN_INTERVAL: f32 = 10.0;

#[derive(Resource)]
pub struct ExplorerSpawnTimer {
    pub time: Timer,
}

impl Default for ExplorerSpawnTimer {
    fn default() -> Self {
        Self {
            time: Timer::from_seconds(EXPLORER_SPAWN_INTERVAL, TimerMode::Repeating),
        }
    }
}

#[derive(Resource)]
pub struct MinerSpawnTimer {
    pub time: Timer,
}

impl Default for MinerSpawnTimer {
    fn default() -> Self {
        Self {
            time: Timer::from_seconds(MINER_SPAWN_INTERVAL, TimerMode::Repeating),
        }
    }
}
