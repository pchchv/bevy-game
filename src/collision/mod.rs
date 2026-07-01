mod tile_type;
mod map;
mod systems;

#[cfg(debug_assertions)]
mod debug;

use bevy::prelude::*;
use crate::state::GameState;

// Re-export commonly used types
pub use tile_type::{TileType, TileMarker};
pub use map::CollisionMap;
pub use systems::CollisionMapBuilt;

#[cfg(debug_assertions)]
pub use debug::DebugCollisionEnabled;

/// Plugin for collision detection functionality
pub struct CollisionPlugin;