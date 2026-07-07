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