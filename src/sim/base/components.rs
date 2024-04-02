use bevy::prelude::*;

#[derive(Component)]
pub struct Base {
    pub pos: Vec2,
    pub iron: u32,
    pub nb_explorer_max: usize,
}
