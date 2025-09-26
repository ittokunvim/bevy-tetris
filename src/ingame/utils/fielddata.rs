use bevy::prelude::*;

use crate::GRID_SIZE;

pub const FIELD_SIZE: Vec2 = Vec2::new(10.0 * GRID_SIZE, 20.0 * GRID_SIZE);
pub const FIELD_POSITION: Vec3 = Vec3::new(0.0, 80.0, -10.0);
pub const FIELD_LEFT_TOP: Vec2 = Vec2::new(
    FIELD_POSITION.x - FIELD_SIZE.x / 2.0 + GRID_SIZE / 2.0, 
    FIELD_POSITION.y + FIELD_SIZE.y / 2.0 - GRID_SIZE / 2.0,
);
