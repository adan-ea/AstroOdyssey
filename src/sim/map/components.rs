use bevy::prelude::*;
use bevy_ecs_tilemap::tiles::TilePos;

#[derive(Component)]
pub struct LabelParent;

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

#[derive(PartialEq)]
pub enum TerrainType {
    Snow,
    Desert,
    Grassland,
    Mushroom,
    Ocean,
    Lake,
    Rocky,
    DenseForest,
    PatchForest,
    Jungle,
}

#[derive(Debug)]
pub struct Tile {
    pub pos: TilePos,
    pub index: TileIndex,
}

#[derive(Deref, Component, Clone, Copy)]
pub struct ChunkPos(pub IVec2);
