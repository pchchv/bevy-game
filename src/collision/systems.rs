use bevy::prelude::*;
use std::collections::{HashMap, hash_map::Entry};

use super::{CollisionMap, TileMarker, TileType};
use crate::config::map::{TILE_SIZE, GRID_X, GRID_Y};

/// Resource to track if collision map has been built.
#[derive(Resource, Default, PartialEq, Eq)]
pub struct CollisionMapBuilt(pub bool);