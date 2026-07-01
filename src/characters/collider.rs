use bevy::prelude::*;
use crate::collision::CollisionMap;
use crate::characters::physics::Velocity;
use crate::config::player::{COLLIDER_RADIUS};

/// A circular collider for collision detection.
/// 
/// The collider position is offset from the entity's transform,
#[derive(Component, Debug, Clone)]
pub struct Collider {
    /// Radius of the circular collider in world units
    pub radius: f32,
    /// Offset from entity center
    pub offset: Vec2,
}

impl Default for Collider {
    fn default() -> Self {
        Self {
            radius: COLLIDER_RADIUS,
            offset: Vec2::ZERO,
        }
    }
}