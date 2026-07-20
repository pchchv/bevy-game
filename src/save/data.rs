use serde::{Serialize, Deserialize};

use crate::combat::PowerType;
use crate::characters::facing::Facing;

pub const MAX_SLOTS: usize = 5;
pub const SAVE_VERSION: u32 = 1;

#[derive(Serialize, Deserialize)]
pub struct SaveFile {
    pub checksum: u64,
    pub data: Vec<u8>,
}

#[derive(Serialize, Deserialize)]
pub struct PlayerSave {
    pub position: [f32; 3],
    pub health_current: f32,
    pub health_max: f32,
    pub power_type: PowerType,
    pub character_name: String,
    pub character_index: usize,
    pub facing: Facing,
}

#[derive(Serialize, Deserialize)]
pub struct EnemySave {
    pub position: [f32; 3],
    pub health_current: f32,
    pub health_max: f32,
    pub character_name: String,
    pub power_type: PowerType,
    pub facing: Facing,
}