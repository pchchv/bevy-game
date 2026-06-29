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

/// Reads player input and updates movement-related components.
pub fn handle_player_input(
    input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(
        &mut CharacterState,
        &mut Velocity,
        &mut Facing,
        &CharacterEntry,
    ), With<Player>>,
) {
    let Ok((mut state, mut velocity, mut facing, character)) = query.single_mut() else {
        return;
    };
    
    // Step 1: Read what keys are pressed
    let direction = read_movement_input(&input);
    let is_running = input.pressed(KeyCode::ShiftLeft) || input.pressed(KeyCode::ShiftRight);
    let wants_jump = input.just_pressed(KeyCode::Space);
    
    // Step 2: Update facing direction (which way the character looks)
    if direction != Vec2::ZERO {
        let new_facing = Facing::from_velocity(direction);
        if *facing != new_facing {
            *facing = new_facing;
        }
    }
    
    // Step 3: Use our state machine to determine the new state
    // This calls the determine_new_state function we wrote earlier
    let new_state = determine_new_state(*state, direction, is_running, wants_jump);
    if *state != new_state {
        *state = new_state;  // This triggers Changed<CharacterState>!
    }
    
    // Step 4: Calculate velocity based on state
    // Idle and Jumping = no movement, Walking/Running = movement
    *velocity = super::physics::calculate_velocity(*state, direction, character);
}