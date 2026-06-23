use bevy::prelude::*;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AnimationType {
    Walk,
    Run,
    Jump
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimationDefinition {
    pub start_row: usize,
    pub frame_count: usize,
    pub frame_time: f32,
    pub directional: bool, // true = 4 rows (one per direction), false = 1 row
}