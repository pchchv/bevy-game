use bevy::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SaveLoadMode {
    Save,
    Load,
}

#[derive(Resource, Default)]
pub struct PendingSaveLoadAction(pub Option<(SaveLoadMode, usize)>);