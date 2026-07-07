use bevy::prelude::*;
use super::power_type::PowerType;

/// Marker for projectile effects
#[derive(Component)]
pub struct ProjectileEffect {
    pub power_type: PowerType,
}