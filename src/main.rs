use bevy::prelude::*;
use crate::player::PlayerPlugin;

mod player;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::WHITE))
        .add_plugins(
            DefaultPlugins.set(AssetPlugin {
                file_path: "src/assets".into(),
                ..default()
            }),
        )
        .add_systems(Startup, setup_camera)
        .add_plugins(PlayerPlugin) // Update this line
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}