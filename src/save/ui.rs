use bevy::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SaveLoadMode {
    Save,
    Load,
}