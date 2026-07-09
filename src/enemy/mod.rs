pub mod ai;
pub mod spawn;
pub mod combat;
pub mod components;

use bevy::prelude::*;
use spawn::EnemiesSpawned;
use crate::state::GameState;
use crate::collision::CollisionMapBuilt;

pub use spawn::spawn_enemy;
pub use components::{AIBehavior, Enemy, EnemyCombat};

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<EnemiesSpawned>()
            // Spawn enemies AFTER collision map is ready (prevents spawning on obstacles)
            .add_systems(
                Update,
                spawn::spawn_test_enemies
                    .run_if(resource_equals(CollisionMapBuilt(true)))
                    .run_if(resource_equals(EnemiesSpawned(false)))
                    .run_if(in_state(GameState::Playing)),
            )
            // Enemy AI and combat systems
            .add_systems(
                Update,
                (ai::enemy_follow_player, combat::enemy_attack)
                    .chain()
                    .run_if(in_state(GameState::Playing)),
            );
    }
}