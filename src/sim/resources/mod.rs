use bevy::prelude::*;

pub mod iron;
pub mod tree;

pub mod components;
pub mod events;
mod systems;

use iron::IronPlugin;
use systems::*;

use self::{events::ResourceSpawnEvent, tree::TreePlugin};

pub struct ResourcesPlugin;

impl Plugin for ResourcesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((IronPlugin, TreePlugin))
            .add_systems(Update, dispatch_resource_spawn)
            .add_event::<ResourceSpawnEvent>();
    }
}
