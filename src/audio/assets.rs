use bevy::prelude::*;

#[derive(Resource)]
pub struct AudioAssets {
    pub menu_music: Handle<AudioSource>,
    pub battle_music: Handle<AudioSource>,
    pub spell_generic: Handle<AudioSource>,
    pub spell_fire: Handle<AudioSource>,
    pub enemy_shoot: Handle<AudioSource>,
    pub hit: Handle<AudioSource>,
    pub player_death: Handle<AudioSource>,
    pub enemy_death: Handle<AudioSource>,
    pub pickup: Handle<AudioSource>,
    pub button_click: Handle<AudioSource>,
    pub jump: Handle<AudioSource>,
}