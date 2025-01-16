use bevy::prelude::*;

use crate::GRID_SIZE;

pub mod collision;
pub mod movement;
pub mod spawn;

const BLOCK_SPEED: f32 = 0.2;
const BLOCK_SIZE: Vec2 = Vec2::splat(GRID_SIZE - 2.0);
const BLOCK_POSITION: Vec2 = Vec2::new(
    -1.0 * GRID_SIZE - GRID_SIZE / 2.0,
    10.0 * GRID_SIZE - GRID_SIZE / 2.0,
);

#[derive(Event)]
pub struct MoveEvent(pub Direction);

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Direction {
    Left,
    Right,
    Bottom,
}

#[derive(Event, Default)]
pub struct CollisionEvent;

#[derive(Event, Default)]
pub struct BottomHitEvent;

#[derive(Event, Default)]
struct SpawnEvent;

#[derive(Component, Default)]
pub struct PlayerBlock;

#[derive(Component)]
#[require(Sprite, Transform, PlayerBlock)]
pub struct Block;

#[allow(dead_code)]
enum BlockType {
    TypeI,
    TypeJ,
    TypeL,
    TypeO,
    TypeS,
    TypeT,
    TypeZ,
}

pub struct BlockPlugin;

impl Plugin for BlockPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<MoveEvent>()
            .add_event::<CollisionEvent>()
            .add_event::<BottomHitEvent>()
            .add_event::<SpawnEvent>()
            .add_plugins(collision::CollisionPlugin)
            .add_plugins(movement::MovementPlugin)
            .add_plugins(spawn::SpawnPlugin)
        ;
    }
}
