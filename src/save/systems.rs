use std::fs;
use super::data::*;

pub fn load_slot_metadata(slot: usize) -> Option<SaveMetadata> {
    let path = meta_file_path(slot);
    let bytes = fs::read(&path).ok()?;
    bincode::deserialize(&bytes).ok()
}