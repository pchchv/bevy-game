use bevy::{prelude::*, sprite::Anchor};
use bevy_procedural_tilemaps::prelude::*;

#[derive(Clone)]
pub struct SpawnableAsset {
    /// Name of the sprite inside our tilemap atlas
    sprite_name: &'static str,
    /// Offset in grid coordinates (for multi-tile objects)
    grid_offset: GridDelta,
    /// Offset in world coordinates (fine positioning)
    offset: Vec3,
    /// Function to add custom components (like collision, physics, etc.)
    components_spawner: fn(&mut EntityCommands),
}

impl SpawnableAsset {
    pub fn with_grid_offset(mut self, offset: GridDelta) -> Self {
        self.grid_offset = offset;
        self
    }
}