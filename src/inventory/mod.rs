use bevy::prelude::*;

use crate::state::GameState;

mod systems;
mod inventory;

use systems::handle_pickups;
pub use inventory::{ItemKind, Pickable, Inventory};

/// Plugin for inventory and pickup functionality.
pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Inventory>()
            .add_systems(
                Update,
                handle_pickups.run_if(in_state(GameState::Playing)),
            );
    }
}