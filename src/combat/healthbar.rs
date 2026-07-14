// use bevy::prelude::*;

/// Marker: this entity is the colored fill of a healthbar.
#[derive(Component)]
pub struct HealthBarForeground;

/// Links a healthbar entity back to its owner character.
#[derive(Component)]
pub struct HealthBarOwner(pub Entity);

/// Green → Yellow → Red, continuous at ratio = 0.5.
fn health_color(ratio: f32) -> Color {
    if ratio >= 0.5 {
        let t = (ratio - 0.5) * 2.0; // 1.0 at full, 0.0 at half
        Color::srgb(1.0 - t * 0.8, 0.8, 0.2)
    } else {
        let t = ratio * 2.0; // 1.0 at half, 0.0 at empty
        Color::srgb(1.0, t * 0.8, 0.2)
    }
}