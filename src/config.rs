//! Centralized configuration constants for the game.

/// Player-related configuration
pub mod player {
    /// Collision radius for the player's collider (in world units)
    pub const COLLIDER_RADIUS: f32 = 16.0;
    /// Z-position for player rendering (above terrain, below UI)
    pub const PLAYER_Z_POSITION: f32 = 20.0;
    /// Visual scale of the player sprite
    pub const PLAYER_SCALE: f32 = 0.8;
}