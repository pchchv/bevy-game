//! Rendering utilities for depth sorting.

use bevy::prelude::*;

use crate::characters::input::Player;
use crate::config::map::{GRID_Y, TILE_SIZE};
use crate::config::player::PLAYER_SCALE;

/// Z-depth constants for proper layering.
/// The tilemap uses `with_z_offset_from_y(true)` which assigns Z based on Y position.
/// We need to match this formula for the player.
const NODE_SIZE_Z: f32 = 1.0;  // Same as tilemap generator
const PLAYER_BASE_Z: f32 = 4.0;  // Match props layer Z range
const PLAYER_Z_OFFSET: f32 = 0.5;  // Small offset to stay above ground props