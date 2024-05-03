use bevy::{prelude::*, utils::HashSet};
use bevy_ecs_tilemap::tiles::TilePos;
use lazy_static::lazy_static;
use std::collections::HashMap;

use crate::sim::resources::components::ResourceType;

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

lazy_static! {
    /// Get the type of tile based on the biome it is in.
    /// The sum of the values MUST be 1.0
    pub static ref BIOME_DATA: HashMap<TerrainType, HashMap<TileIndex, f32>> = {
        use TerrainType::*;
        use TileIndex::*;

        let mut m = HashMap::new();

        let mut grass = HashMap::new();
        grass.insert(Grass, 0.995);
        grass.insert(Dirt, 0.005);
        m.insert(Plains, grass);

        let mut beach = HashMap::new();
        beach.insert(Sand, 0.98);
        beach.insert(Rock, 0.02);
        m.insert(Beach, beach);

        let mut jungle = HashMap::new();
        jungle.insert(JungleTile, 1.0);
        m.insert(Jungle, jungle);

        let mut desert = HashMap::new();
        desert.insert(Sand, 0.97);
        desert.insert(Rock, 0.03);
        m.insert(Desert, desert);

        let mut lake = HashMap::new();
        lake.insert(ShallowWater, 1.0);
        m.insert(Lake, lake);

        let mut mountain = HashMap::new();
        mountain.insert(Rock, 0.95);
        mountain.insert(Snow, 0.05);
        m.insert(Mountain, mountain);

        let mut snow = HashMap::new();
        snow.insert(Snow, 0.95);
        snow.insert(Rock, 0.03);
        snow.insert(Grass, 0.02);
        m.insert(Tundra, snow);

        let mut mushroom = HashMap::new();
        mushroom.insert(MushroomTile, 1.0);
        m.insert(Mushroom, mushroom);

        let mut ocean = HashMap::new();
        ocean.insert(DeepWater, 1.0);
        m.insert(Ocean, ocean);

        m
    };
}

lazy_static! {
    pub static ref RESOURCES_DATA: HashMap<TerrainType, HashMap<ResourceType, f32>> = {
        use ResourceType::*;
        use TerrainType::*;

        let mut m = HashMap::new();

        let mut grass = HashMap::new();
        grass.insert(PineTree, 0.03);
        grass.insert(RoundTree, 0.03);
        grass.insert(Stone, 0.01);
        grass.insert(Iron, 0.01);
        m.insert(Plains, grass);

        let mut jungle = HashMap::new();
        jungle.insert(PineTree, 0.1);
        jungle.insert(RoundTree, 0.1);
        jungle.insert(Iron, 0.01);
        m.insert(Jungle, jungle);

        let mut desert = HashMap::new();
        desert.insert(Cactus, 0.01);
        desert.insert(Stone, 0.02);
        desert.insert(Iron, 0.01);
        m.insert(Desert, desert);

        let mut mountain = HashMap::new();
        mountain.insert(Stone, 0.05);
        mountain.insert(Iron, 0.05);
        mountain.insert(PineTree, 0.01);
        m.insert(Mountain, mountain);

        let mut snow = HashMap::new();
        m.insert(Tundra, snow);

        let mut mushroom = HashMap::new();
        mushroom.insert(Crystal, 0.03);
        mushroom.insert(BlueTree, 0.03);

        m.insert(Mushroom, mushroom);

        m
    };
}
