use bevy::prelude::*;
use super::components::{AIBehavior, Enemy, EnemyCombat, EnemyPath};
use crate::config::enemy::{ENEMY_SCALE, ENEMY_Z_POSITION};
use crate::config::player::COLLIDER_RADIUS;
use crate::collision::CollisionMap;
use crate::characters::{
    animation::{AnimationController, AnimationTimer, DEFAULT_ANIMATION_FRAME_TIME},
    collider::Collider,
    config::{CharacterEntry, CharactersList},
    facing::Facing,
    physics::Velocity,
    spawn::CharactersListResource,
    state::CharacterState,
};

/// Spawn an enemy at the given position.
pub fn spawn_enemy(
    commands: &mut Commands,
    asset_server: &AssetServer,
    atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
    characters_list: &CharactersList,
    position: Vec3,
    character_name: &str,
) -> Option<Entity> {
    // Find the character entry by name
    let character_entry = characters_list
        .characters
        .iter()
        .find(|c| c.name == character_name)?;

    // Create atlas layout
    let max_row = character_entry.calculate_max_animation_row();
    let layout = atlas_layouts.add(TextureAtlasLayout::from_grid(
        UVec2::splat(character_entry.tile_size),
        character_entry.atlas_columns as u32,
        (max_row + 1) as u32,
        None,
        None,
    ));
    // Load texture
    let texture = asset_server.load(&character_entry.texture_path);
    // Create sprite
    let sprite = Sprite::from_atlas_image(texture, TextureAtlas { layout, index: 0 });
    // Spawn enemy entity with all necessary components
    let entity = commands
        .spawn((
            Enemy,
            sprite,
            Transform::from_translation(position).with_scale(Vec3::splat(ENEMY_SCALE)),
            GlobalTransform::default(),
            AnimationController::default(),
            CharacterState::default(),
            Velocity::default(),
            Facing::default(),
            Collider::default(),
            EnemyCombat::default(),
            AIBehavior::default(),
            EnemyPath::default(),  // Add this line
            AnimationTimer(Timer::from_seconds(
                DEFAULT_ANIMATION_FRAME_TIME,
                TimerMode::Repeating,
            )),
            character_entry.clone(),
        ))
        .id();

    info!("Spawned enemy '{}' at {:?}", character_name, position);

    Some(entity)
}

/// Validate and adjust spawn position to ensure it's on a walkable tile.
fn get_valid_spawn_position(collision_map: &CollisionMap, desired_pos: Vec2) -> Vec2 {
    // Use the same radius as the runtime collision system
    if collision_map.is_circle_clear(desired_pos, COLLIDER_RADIUS) {
        return desired_pos;
    }

    // Find nearest position where the full collider circle is clear
    if let Some(clear_pos) = collision_map.find_nearest_clear_position(desired_pos, COLLIDER_RADIUS) {
        info!(
            "Adjusted spawn from {:?} to {:?} (was on obstacle)",
            desired_pos, clear_pos
        );
        return clear_pos;
    }

    // Fallback to original (shouldn't happen in a valid map)
    warn!("Could not find walkable spawn position near {:?}", desired_pos);
    desired_pos
}