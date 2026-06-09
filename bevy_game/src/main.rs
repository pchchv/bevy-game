use bevy::prelude::*;

fn main() {
    println!("Hello, world!");
}

fn setup(mut commands: Commands) {
	commands.spawn(Camera2d);
}