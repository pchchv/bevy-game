use bevy_procedural_tilemaps::prelude::*;

pub struct DirtLayerSockets {
    pub layer_up: Socket,      // What can sit on top of dirt
    pub layer_down: Socket,     // What dirt can sit on
    pub material: Socket,       // What dirt connects to horizontally
}