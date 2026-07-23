use bevy::prelude::*;
use crate::combat::PowerType;

#[derive(Event, Clone, Copy)]
pub enum SfxKind {
    PlayerShoot(PowerType),
    EnemyShoot,
    Hit,
    PlayerDeath,
    EnemyDeath,
    Pickup,
    ButtonClick,
    Jump,
}