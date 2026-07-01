use bevy::prelude::*;
use super::CollisionMap;
use crate::characters::input::Player;
use crate::characters::collider::Collider;

/// Resource to toggle debug visualization.
#[derive(Resource, Default)]
pub struct DebugCollisionEnabled(pub bool);

/// Toggle collision debug visualization with F3 key.
pub fn toggle_debug_collision(keyboard: Res<ButtonInput<KeyCode>>, mut debug_enabled: ResMut<DebugCollisionEnabled>) {
    if keyboard.just_pressed(KeyCode::F3) {
        debug_enabled.0 = !debug_enabled.0;
        if debug_enabled.0 {
            info!("🔍 Collision debug ENABLED (F3 to toggle)");
        } else {
            info!("Collision debug disabled");
        }
    }
}