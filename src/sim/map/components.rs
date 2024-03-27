use bevy::prelude::*;

#[derive(Component)]
pub struct Tile {
    pub pos: (i32, i32),
    pub sprite: usize,
    pub z_index: i32,
    pub blocked: bool,
    pub known: bool,
}

impl Tile {
    // Creates a new Tile instance.
    pub fn new(pos: (i32, i32), sprite: usize, z_index: i32, blocked: bool) -> Self {
        Self {
            pos,
            sprite,
            z_index,
            blocked,
            known: false,
        }
    }
}