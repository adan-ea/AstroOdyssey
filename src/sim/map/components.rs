use bevy::prelude::*;

#[derive(Component)]
pub struct TileComponent;

pub struct Tile {
    pub pos: (i32, i32),
    pub sprite: usize,
    pub z_index: i32,
}

impl Tile {
    // Creates a new Tile instance.
    pub fn new(pos: (i32, i32), sprite: usize, z_index: i32) -> Self {
        Self {
            pos,
            sprite,
            z_index,
        }
    }
}