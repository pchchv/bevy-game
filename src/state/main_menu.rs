use bevy::prelude::*;

#[derive(Component)]
pub struct MainMenuScreen;

#[derive(Component)]
pub enum MainMenuButton {
    NewGame,
    LoadGame,
    Quit,
}