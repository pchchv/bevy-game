use bevy::prelude::*;

/// Tile types for collision detection.
/// Each type has different walkability and collision behavior.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum TileType {
    // Walkable terrain
    #[default]
    Empty,
    Dirt,
    Grass,
    YellowGrass,
    Shore,  // Water edges (walkable)
    // Non-walkable obstacles
    Water,
    Tree,
    Rock,
}

impl TileType {
    /// Check if this tile type allows movement through it.
    pub fn is_walkable(&self) -> bool {
        !matches!(self, TileType::Water | TileType::Tree | TileType::Rock)
    }
}