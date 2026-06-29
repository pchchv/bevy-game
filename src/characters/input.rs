use bevy::prelude::*;
use super::{
    state::CharacterState,
    physics::Velocity,
    facing::Facing,
    config::CharacterEntry,
    animation::{AnimationController, AnimationTimer},
};

#[derive(Component)]
pub struct Player;

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

fn determine_new_state(current: CharacterState, direction: Vec2, is_running: bool, wants_jump: bool) -> CharacterState {
    match current {
        // Can't transition out of jumping until it completes
        CharacterState::Jumping => CharacterState::Jumping,
        
        // Jump takes priority when grounded
        _ if wants_jump && current.is_grounded() => CharacterState::Jumping,
        
        // Movement states
        _ if direction != Vec2::ZERO => {
            if is_running { CharacterState::Running } else { CharacterState::Walking }
        }
        
        // Default to idle
        _ => CharacterState::Idle,
    }
}