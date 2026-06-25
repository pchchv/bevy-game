use bevy::prelude::*;
use crate::characters::animation::*;
use crate::characters::config::{CharacterEntry, AnimationType};

/// Read directional input and return a direction vector
fn read_movement_input(input: &ButtonInput<KeyCode>) -> Vec2 {
    const MOVEMENT_KEYS: [(KeyCode, Vec2); 4] = [
        (KeyCode::ArrowLeft, Vec2::NEG_X),
        (KeyCode::ArrowRight, Vec2::X),
        (KeyCode::ArrowUp, Vec2::Y),
        (KeyCode::ArrowDown, Vec2::NEG_Y),
    ];
    
    MOVEMENT_KEYS.iter()
        .filter(|(key, _)| input.pressed(*key))
        .map(|(_, dir)| *dir)
        .sum()
}

fn calculate_movement_speed(character: &CharacterEntry, is_running: bool) -> f32 {
    if is_running {
        character.base_move_speed * character.run_speed_multiplier
    } else {
        character.base_move_speed
    }
}