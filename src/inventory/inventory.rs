use std::fmt;
use std::collections::HashMap;
use bevy::prelude::*;

use crate::config::pickup::DEFAULT_RADIUS;

/// Types of items that can be collected.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ItemKind {
    Plant1,
    Plant2,
    Plant3,
    Plant4,
}