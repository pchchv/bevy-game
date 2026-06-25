use bevy::prelude::*;
use crate::characters::animation::*;
use crate::characters::config::{CharacterEntry, CharactersList};
use crate::characters::movement::Player;

const PLAYER_SCALE: f32 = 0.8;
const PLAYER_Z_POSITION: f32 = 20.0;

#[derive(Resource, Default)]
pub struct CurrentCharacterIndex {
    pub index: usize,
}

#[derive(Resource)]
pub struct CharactersListResource {
    pub handle: Handle<CharactersList>,
}

/// Create a texture atlas layout for a character
fn create_character_atlas_layout(atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>, character_entry: &CharacterEntry) -> Handle<TextureAtlasLayout> {
    let max_row = character_entry.calculate_max_animation_row();
    atlas_layouts.add(TextureAtlasLayout::from_grid(
        UVec2::splat(character_entry.tile_size),
        character_entry.atlas_columns as u32,
        (max_row + 1) as u32,
        None,
        None,
    ))
}