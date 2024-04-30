use bevy::{prelude::*, utils::HashSet};
use bevy_ecs_tilemap::tiles::TilePos;

#[derive(Debug, Copy, Clone)]
pub enum TileIndex {
    Dirt = 0,
    Grass = 10,
    WaterOcean = 20,
    WaterLake = 21,
    Desert = 30,
    Mushroom = 40,
    Rock = 50,
    Jungle = 60,
    Snow = 70,
}

#[derive(PartialEq, Copy, Debug, Eq, Clone)]
pub enum TerrainType {
    Snow,
    Desert,
    Grassland,
    Mushroom,
    Ocean,
    Lake,
    Rocky,
    Jungle,
}

#[derive(Debug)]
pub struct Tile {
    pub pos: TilePos,
    pub index: TileIndex,
}

impl Tile {
    pub fn new(pos: TilePos, index: TileIndex) -> Self {
        Self { pos, index }
    }
}

#[derive(Deref, Component, Clone, Copy)]
pub struct ChunkPos(pub IVec2);

#[derive(Default, Debug, Resource)]
pub struct ChunkManager {
    pub spawned_chunks: HashSet<IVec2>,
}
