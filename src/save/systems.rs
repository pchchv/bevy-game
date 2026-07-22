use std::fs;
use super::data::*;

pub fn load_slot_metadata(slot: usize) -> Option<SaveMetadata> {
    let path = meta_file_path(slot);
    let bytes = fs::read(&path).ok()?;
    
    bincode::serde::decode_from_slice(&bytes, bincode::config::standard())
        .map(|(data, _)| data)
        .ok()
}

pub fn load_save_data(slot: usize) -> Result<SaveData, String> {
    let path = save_file_path(slot);
    let file_bytes = fs::read(&path).map_err(|e| format!("Read error: {}", e))?;
    let (save_file, _): (SaveFile, usize) = bincode::serde::decode_from_slice(&file_bytes, bincode::config::standard())
    .map_err(|e| format!("Deserialize error: {}", e))?;
    // Verify checksum
    let computed = compute_checksum(&save_file.data);
    if computed != save_file.checksum {
        return Err("Save file corrupted or tampered with".into());
    }

    let (save_data, _bytes_read): (SaveData, usize) = bincode::serde::decode_from_slice(&save_file.data, bincode::config::standard()).map_err(|e| format!("Data deserialize error: {}", e))?;
    if save_data.version != SAVE_VERSION {
        return Err(format!("Incompatible save version: {} (expected {})", save_data.version, SAVE_VERSION));
    }

    Ok(save_data)
}