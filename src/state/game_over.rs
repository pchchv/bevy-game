use bevy::prelude::*;

use crate::enemy::Enemy;
use crate::enemy::spawn::EnemiesSpawned;
use crate::characters::spawn::PlayerSpawned;
use crate::combat::healthbar::HealthBarOwner;
use crate::combat::systems::{Projectile, ProjectileEffect};
use crate::particles::components::{Particle, ParticleEmitter};

#[derive(Component)]
pub struct GameOverScreen;

/// Despawns all gameplay entities and resets spawn flags so they re-trigger.
pub fn cleanup_game_world(
    mut commands: Commands,
    enemies: Query<Entity, With<Enemy>>,
    projectiles: Query<Entity, With<Projectile>>,
    projectile_effects: Query<Entity, With<ProjectileEffect>>,
    emitters: Query<Entity, With<ParticleEmitter>>,
    particles: Query<Entity, With<Particle>>,
    healthbars: Query<Entity, With<HealthBarOwner>>,
    mut player_spawned: ResMut<PlayerSpawned>,
    mut enemies_spawned: ResMut<EnemiesSpawned>,
) {
    for entity in enemies.iter() {
        commands.entity(entity).despawn();
    }

    for entity in projectiles.iter() {
        commands.entity(entity).despawn();
    }

    for entity in projectile_effects.iter() {
        commands.entity(entity).despawn();
    }

    for entity in emitters.iter() {
        commands.entity(entity).despawn();
    }

    for entity in particles.iter() {
        commands.entity(entity).despawn();
    }

    for entity in healthbars.iter() {
        commands.entity(entity).despawn();
    }

    player_spawned.0 = false;
    enemies_spawned.0 = false;
}

pub fn spawn_game_over_screen(mut commands: Commands) {
    commands
        .spawn((
            GameOverScreen,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.85)),
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("GAME OVER\n\nPress R to restart"),
                TextFont {
                    font_size: FontSize::Px(48.0),
                    ..default()
                },
                TextColor(Color::WHITE),
                TextLayout::justify(Justify::Center),
            ));
        });

    info!("Game over screen spawned");
}

pub fn despawn_game_over_screen(mut commands: Commands, query: Query<Entity, With<GameOverScreen>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
    info!("Game over screen despawned");
}