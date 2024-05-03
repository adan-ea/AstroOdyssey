use bevy::prelude::*;

use super::components::Resource;

#[derive(Event)]
pub struct ResourceSpawnEvent {
    pub resources: Vec<Resource>,
}

impl IntoIterator for ResourceSpawnEvent {
    type Item = Resource;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.resources.into_iter()
    }
}
