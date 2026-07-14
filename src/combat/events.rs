use bevy::prelude::*;

/// Event triggered when an entity's health reaches zero.
#[derive(Event)]
pub struct EntityDeath {
    pub entity: Entity,
}