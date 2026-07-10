pub mod state;
pub mod input;
pub mod spawn;
pub mod config;
pub mod facing;
pub mod physics;
pub mod collider;
pub mod animation;
mod rendering;

use bevy::prelude::*;
use spawn::PlayerSpawned;
use config::CharactersList;
use crate::state::GameState;
use crate::collision::CollisionMapBuilt;
use bevy_common_assets::ron::RonAssetPlugin;

pub struct CharactersPlugin;

impl Plugin for CharactersPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RonAssetPlugin::<CharactersList>::new(&["characters.ron"]))
            .init_resource::<spawn::CurrentCharacterIndex>()
            .init_resource::<PlayerSpawned>()
            // Load character assets at startup (before collision map)
            .add_systems(Startup, spawn::load_character_assets)
            // Spawn player at valid position AFTER collision map is built
            .add_systems(
                Update,
                spawn::spawn_player_at_valid_position
                    .run_if(resource_equals(CollisionMapBuilt(true)))
                    .run_if(resource_equals(PlayerSpawned(false)))
                    .run_if(in_state(GameState::Playing)),
            )
            .add_systems(
                Update,
                (
                    input::handle_player_input,
                    spawn::switch_character,
                    input::update_jump_state,
                    animation::on_state_change_update_animation,
                    collider::validate_movement,
                    collider::resolve_entity_collisions,
                    physics::apply_velocity,
                    rendering::update_character_depth,
                    animation::animations_playback,
                )
                    .chain()
                    .run_if(in_state(GameState::Playing)),
            );
    }
}