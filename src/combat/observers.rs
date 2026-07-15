use bevy::prelude::*;
use super::health::Health;
use super::events::ProjectileHit;

/// Observer that handles projectile hits by applying damage to the target.
pub fn on_projectile_hit(hit: On<ProjectileHit>, mut healths: Query<&mut Health>, mut commands: Commands) {
    let Ok(mut health) = healths.get_mut(hit.target) else {
        return;
    };

    health.take_damage(&mut commands, hit.target, hit.damage);
    info!(
        "{:?} hit for {} damage! HP: {:.0}/{:.0}",
        hit.power_type, hit.damage, health.current, health.max
    );
}