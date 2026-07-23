use bevy::prelude::*;
use super::assets::AudioAssets;

/// Marker for music entities so we can find and stop them.
#[derive(Component)]
pub struct MusicTrack;

pub fn start_menu_music(mut commands: Commands, audio_assets: Option<Res<AudioAssets>>, existing: Query<Entity, With<MusicTrack>>) {
    // 1. Stop any currently playing music
    despawn_all_music(&mut commands, &existing);

    // 2. Safely get our loaded audio assets
    let Some(audio) = audio_assets else {
        return
    };

    // 3. Spawn the new audio player
    commands.spawn((MusicTrack, AudioPlayer::new(audio.menu_music.clone()), PlaybackSettings::LOOP));
}

pub fn start_battle_music(mut commands: Commands, audio_assets: Option<Res<AudioAssets>>, existing: Query<Entity, With<MusicTrack>>) {
    despawn_all_music(&mut commands, &existing);

    let Some(audio) = audio_assets else { return; };
    commands.spawn((MusicTrack, AudioPlayer::new(audio.battle_music.clone()), PlaybackSettings::LOOP));
}

pub fn stop_all_music(mut commands: Commands, existing: Query<Entity, With<MusicTrack>>) {
    despawn_all_music(&mut commands, &existing);
}

// Helper for removing duplicates.
fn despawn_all_music(commands: &mut Commands, query: &Query<Entity, With<MusicTrack>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}