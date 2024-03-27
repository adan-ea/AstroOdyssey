use bevy::prelude::*;

#[derive(Component)]
pub struct Base {
    pub pos: (i32, i32),
    pub iron: u32,
    pub nb_explorer_max: usize,
}
