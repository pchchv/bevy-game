use super::components::*;
use super::material::ParticleMaterial;
use bevy::prelude::*;
use rand::Rng;

/// System to update particle emitters and spawn new particles
pub fn update_emitters(
    mut commands: Commands,
    time: Res<Time>,
    mut emitters: Query<(Entity, &mut ParticleEmitter, &GlobalTransform)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ParticleMaterial>>,
) {
    let mut rng = rand::thread_rng();
    for (entity, mut emitter, global_transform) in emitters.iter_mut() {
        if !emitter.active {
            continue;
        }

        // Handle one-shot emitters
        if emitter.one_shot && emitter.has_spawned {
            emitter.active = false;
            continue;
        }

        emitter.spawn_timer.tick(time.delta());
        if emitter.spawn_timer.just_finished() {
            emitter.has_spawned = true;
            // Spawn particles
            for i in 0..emitter.particles_per_spawn {
                spawn_particle(
                    &mut commands,
                    &emitter.particle_config,
                    global_transform,
                    &mut rng,
                    &mut meshes,
                    &mut materials,
                    Some(entity),
                    i,
                );
            }

            if emitter.one_shot {
                emitter.active = false;
            }
        }
    }
}