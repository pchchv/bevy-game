use bevy::prelude::*;

/// Marker for music entities so we can find and stop them.
#[derive(Component)]
pub struct MusicTrack;

pub fn stop_all_music(mut commands: Commands, existing: Query<Entity, With<MusicTrack>>) {
    for entity in existing.iter() {
        commands.entity(entity).despawn();
    }
}