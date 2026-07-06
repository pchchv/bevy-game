use bevy::prelude::*;

/// A single particle in the particle system
#[derive(Component, Clone)]
pub struct Particle {
    pub velocity: Vec3,           // Movement speed and direction (units/sec)
    pub lifetime: f32,             // Remaining time before death (seconds)
    pub max_lifetime: f32,         // Original lifetime for progress calculation
    pub scale: f32,                // Current size multiplier
    pub angular_velocity: f32,     // Rotation speed (radians/sec)
    pub acceleration: Vec3,        // Forces like gravity (units/sec²)
    // Color curve support (start → mid → end)
    pub start_color: Color,        // Color at birth (0% lifetime)
    pub mid_color: Color,          // Color at midpoint (50% lifetime)
    pub end_color: Color,          // Color at death (100% lifetime)
    // Scale curve support
    pub start_scale: f32,          // Size at birth
    pub end_scale: f32,            // Size at death (usually smaller)
}