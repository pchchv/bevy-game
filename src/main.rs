use bevy::prelude::*;

mod player;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::WHITE)) // We have updated the bg color to white
        .add_plugins(
            DefaultPlugins.set(AssetPlugin {
                file_path: "src/assets".into(),
                ..default()
            }),
        )
        .add_systems(Startup, setup_camera)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}
