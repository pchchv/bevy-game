use bevy::prelude::*;
use crate::particles::components::{EmissionShape, ParticleConfig};

/// The different magical powers available
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PowerType {
    #[default]
    Fire,
    Arcane,
    Shadow,
    Poison,
}

/// Visual configuration for a power - decoupled from behavior
#[derive(Clone)]
pub struct PowerVisuals {
    pub primary: ParticleConfig,
    pub core: Option<ParticleConfig>,
    pub particles_per_spawn: u32,
    pub core_particles_per_spawn: u32,
}

impl PowerType {
    /// Get visual configuration for this power
    pub fn visuals(&self, direction: Vec3) -> PowerVisuals {
        match self {
            PowerType::Fire => Self::fire_visuals(direction),
            PowerType::Arcane => Self::arcane_visuals(direction),
            PowerType::Shadow => Self::shadow_visuals(direction),
            PowerType::Poison => Self::poison_visuals(direction),
        }
    }
}