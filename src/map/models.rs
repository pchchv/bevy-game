use bevy_procedural_tilemaps::prelude::*;
use crate::map::assets::SpawnableAsset;

/// Utility wrapper that ensures model declarations and their asset bindings stay aligned.
pub struct TerrainModelBuilder {
    pub models: ModelCollection<Cartesian3D>,
    pub assets: Vec<Vec<SpawnableAsset>>,
}