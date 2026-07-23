use bevy::prelude::*;

/// Marker for music entities so we can find and stop them.
#[derive(Component)]
pub struct MusicTrack;