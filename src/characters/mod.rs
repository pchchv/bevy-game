pub mod state;
pub mod input;
pub mod spawn;
pub mod config;
pub mod facing;
pub mod physics;
pub mod collider;
pub mod animation;

use crate::state::GameState;

use bevy::prelude::*;
use bevy_common_assets::ron::RonAssetPlugin;
use config::CharactersList;

pub struct CharactersPlugin;

impl Plugin for CharactersPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RonAssetPlugin::<CharactersList>::new(&["characters.ron"]))
        .init_resource::<spawn::CurrentCharacterIndex>()
        .add_systems(Startup, spawn::spawn_player)
        .add_systems(Update, (
            // 1. Input determines state + velocity + facing
            input::handle_player_input,
            spawn::switch_character,
            input::update_jump_state,
            
            // 2. State changes trigger animation updates
            animation::on_state_change_update_animation,
            
            // 3. Collision validation adjusts velocity  <-- Line update alert!
            collider::validate_movement,
            
            // 4. Physics applies velocity to transform
            physics::apply_velocity,
            
            // 5. Animation ticks frames
            animation::animations_playback,
        ).chain().run_if(in_state(GameState::Playing)));
    }
}