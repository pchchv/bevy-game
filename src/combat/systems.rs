use super::player_combat::PlayerCombat;
use super::power_type::{PowerType, PowerVisuals};
use crate::characters::facing::Facing;
use crate::characters::input::Player;
use crate::particles::components::ParticleEmitter;
use bevy::prelude::*;

/// Marker for projectile effects
#[derive(Component)]
pub struct ProjectileEffect {
    pub power_type: PowerType,
}

pub fn handle_power_input(
    mut commands: Commands,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut player_query: Query<(&GlobalTransform, &Facing, &mut PlayerCombat), With<Player>>,
) {
    let Ok((global_transform, facing, mut combat)) = player_query.single_mut() else {
        return;
    };

    combat.cooldown.tick(time.delta());
    let ctrl_pressed = input.just_pressed(KeyCode::ControlLeft) || input.just_pressed(KeyCode::ControlRight);
    if !ctrl_pressed {
        return;
    }

    // Only fire if cooldown has elapsed
    if combat.cooldown.elapsed_secs() < combat.cooldown.duration().as_secs_f32() {
        return;
    }

    combat.cooldown.reset();
    let position: Vec3 = global_transform.translation();
    let direction = facing_to_vec3(facing);
    let spawn_position = position + direction * 5.0;
    // Get visuals from power type
    let visuals = combat.power_type.visuals(direction);
    spawn_projectile(&mut commands, spawn_position, combat.power_type, &visuals);

    info!("{:?} projectile fired!", combat.power_type);
}