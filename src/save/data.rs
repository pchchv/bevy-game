use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct SaveFile {
    pub checksum: u64,
    pub data: Vec<u8>,
}