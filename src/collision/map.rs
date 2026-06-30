use bevy::prelude::*;
use super::TileType;

/// Collision map resource that stores walkability information.
/// Provides efficient spatial queries for movement validation.
#[derive(Resource)]
pub struct CollisionMap {
    /// Flat array of tile types (row-major order)
    tiles: Vec<TileType>,
    /// Grid dimensions
    width: i32,
    height: i32,
    /// Size of each tile in world units
    tile_size: f32,
    /// World position of grid origin (bottom-left corner)
    origin_x: f32,
    origin_y: f32,
}

impl CollisionMap {
    /// Create a new collision map with specified dimensions and origin.
    pub fn new(width: i32, height: i32, tile_size: f32, origin_x: f32, origin_y: f32) -> Self {
        let size = (width * height) as usize;
        Self {
            tiles: vec![TileType::Empty; size],
            width,
            height,
            tile_size,
            origin_x,
            origin_y,
        }
    }

    /// Convert 2D grid coordinates to 1D array index.
    #[inline]
    fn xy_to_idx(&self, x: i32, y: i32) -> usize {
        (y * self.width + x) as usize
    }

    /// Check if grid coordinates are within bounds.
    #[inline]
    pub fn in_bounds(&self, x: i32, y: i32) -> bool {
        x >= 0 && x < self.width && y >= 0 && y < self.height
    }

    /// Convert world position to grid coordinates.
    pub fn world_to_grid(&self, world_pos: Vec2) -> IVec2 {
        let grid_x = ((world_pos.x - self.origin_x) / self.tile_size).floor() as i32;
        let grid_y = ((world_pos.y - self.origin_y) / self.tile_size).floor() as i32;
        IVec2::new(grid_x, grid_y)
    }

    /// Convert grid coordinates to world position (tile center).
    pub fn grid_to_world(&self, grid_x: i32, grid_y: i32) -> Vec2 {
        Vec2::new(
            self.origin_x + (grid_x as f32 + 0.5) * self.tile_size,
            self.origin_y + (grid_y as f32 + 0.5) * self.tile_size,
        )
    }
    
    /// Get the tile type at grid coordinates.
    pub fn get_tile(&self, x: i32, y: i32) -> Option<TileType> {
        if self.in_bounds(x, y) {
            Some(self.tiles[self.xy_to_idx(x, y)])
        } else {
            None
        }
    }

    /// Set a tile at grid coordinates.
    pub fn set_tile(&mut self, x: i32, y: i32, tile_type: TileType) {
        if self.in_bounds(x, y) {
            let idx = self.xy_to_idx(x, y);
            self.tiles[idx] = tile_type;
        }
    }

    /// Check if a grid position is walkable.
    pub fn is_walkable(&self, x: i32, y: i32) -> bool {
        self.get_tile(x, y).map_or(false, |t| t.is_walkable())
    }
}