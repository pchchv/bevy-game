mod power_type;
mod player_combat;
pub mod healthbar;
pub mod systems;
pub mod health;

pub use health::Health;

pub use player_combat::PlayerCombat;
pub use power_type::{PowerType, PowerVisuals};
pub use systems::{debug_switch_power, handle_power_input, spawn_projectile};

use bevy::prelude::*;
use crate::state::GameState;

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(
                Update,
                (
                    handle_power_input,
                    debug_switch_power,
                    healthbar::spawn_healthbars,
                    healthbar::update_healthbars,
                )
                    .chain()
                    .run_if(in_state(GameState::Playing)),
            );
    }
}