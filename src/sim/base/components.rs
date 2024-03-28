use bevy::prelude::*;

#[derive(Component)]
pub struct Base {
    pub pos: (f32, f32),
    pub iron: u32,
    pub nb_explorer_max: usize,
}
