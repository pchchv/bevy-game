use bevy::prelude::*;

/// Health component for any damageable entity (player, enemies).
#[derive(Component)]
pub struct Health {
    pub current: f32,
    pub max: f32,
}

impl Health {
    pub fn new(max: f32) -> Self {
        Self { current: max, max }
    }

    pub fn is_alive(&self) -> bool {
        self.current > 0.0
    }

    /// Returns health as a ratio in [0, 1].
    pub fn ratio(&self) -> f32 {
        self.current / self.max
    }
}