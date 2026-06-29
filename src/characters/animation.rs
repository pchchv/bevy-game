use bevy::prelude::*;
use crate::characters::facing::Facing;
use crate::characters::config::{CharacterEntry, AnimationType};
// use crate::characters::state::CharacterState;

// Default animation timing (10 FPS = 0.1 seconds per frame)
pub const DEFAULT_ANIMATION_FRAME_TIME: f32 = 0.1;

// Component that holds animation configuration
#[derive(Component, Default)]
pub struct AnimationController {
    pub current_animation: AnimationType,
}

impl AnimationController {
    /// Get the animation clip for the current animation and facing direction.
    /// `facing` is passed in since it's now a separate component.
    pub fn get_clip(&self, config: &CharacterEntry, facing: Facing) -> Option<AnimationClip> {
        let def = config.animations.get(&self.current_animation)?;
        let row = if def.directional {
            def.start_row + facing.direction_index()
        } else {
            def.start_row
        };
        
        Some(AnimationClip::new(row, def.frame_count, config.atlas_columns))
    }
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

#[derive(Clone, Copy)]
pub struct AnimationClip {
    first: usize,
    last: usize,
}

impl AnimationClip {
    pub fn new(row: usize, frame_count: usize, atlas_columns: usize) -> Self {
        let first = row * atlas_columns;
        Self {
            first,
            last: first + frame_count - 1,
        }
    }
    
    pub fn start(self) -> usize {
        self.first
    }
    
    // Check if a frame index belongs to this clip
    pub fn contains(self, index: usize) -> bool {
        (self.first..=self.last).contains(&index)
    }
    
    // Calculate the next frame, looping back to start if needed
    pub fn next(self, index: usize) -> usize {
        if index >= self.last {
            self.first
        } else {
            index + 1
        }
    }
    
    // Check if animation has completed (used for non-looping animations like Jump)
    pub fn is_complete(self, current_index: usize, timer_finished: bool) -> bool {
        current_index >= self.last && timer_finished
    }
}