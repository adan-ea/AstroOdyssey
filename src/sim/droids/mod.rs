use bevy::prelude::*;

use self::explorer::ExplorerPlugin;

pub mod components;
pub mod explorer;

pub struct DroidsPlugin;

impl Plugin for DroidsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ExplorerPlugin);
    }
}
