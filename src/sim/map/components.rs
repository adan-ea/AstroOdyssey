use bevy::{prelude::*, utils::HashSet};
use bevy_ecs_tilemap::tiles::TilePos;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum TileIndex {
    Dirt = 0,
    Grass = 10,
    DeepWater = 20,
    ShallowWater = 21,
    Sand = 30,
    MushroomTile = 40,
    Rock = 50,
    JungleTile = 60,
    Snow = 70,
}

#[derive(PartialEq, Copy, Debug, Eq, Clone, Hash)]
pub enum TerrainType {
    Tundra,
    Beach,
    Desert,
    Plains,
    Mushroom,
    Ocean,
    Lake,
    Mountain,
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

#[derive(Debug, Resource)]
pub struct MapManager {
    pub spawned_chunks: HashSet<IVec2>,
    pub seed: u32,
}

impl Default for MapManager {
    fn default() -> Self {
        Self {
            spawned_chunks: HashSet::new(),
            seed: 0,
        }
    }
}
