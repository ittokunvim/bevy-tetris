use bevy::prelude::*;
use rand::{
    distributions::Standard,
    prelude::Distribution,
    Rng,
};

use crate::{
    GRID_SIZE,
    AppState,
};

pub mod collision;
pub mod movement;
pub mod rotation;
mod spawn;

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
pub struct RotationEvent;

#[derive(Event, Default)]
pub struct CollisionEvent;

#[derive(Event, Default)]
pub struct SpawnEvent;

#[derive(Resource)]
pub struct CurrentBlock {
    id: usize,
    block: BlockType,
    init_pos: Vec2,
}

#[derive(Component)]
pub struct PlayerBlock(pub usize);

#[derive(Component)]
#[require(Sprite, Transform)]
pub struct Block;

#[allow(dead_code)]
#[derive(Copy, Clone, PartialEq, Debug)]
enum BlockType {
    TypeI,
    TypeJ,
    TypeL,
    TypeO,
    TypeS,
    TypeT,
    TypeZ,
}

impl Distribution<BlockType> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> BlockType {
        let index: u8 = rng.gen_range(0..7);

        match index {
            0 => BlockType::TypeI,
            1 => BlockType::TypeJ,
            2 => BlockType::TypeL,
            3 => BlockType::TypeO,
            4 => BlockType::TypeS,
            5 => BlockType::TypeT,
            6 => BlockType::TypeZ,
            _ => unreachable!(),
        }
    }
}

impl CurrentBlock {
    fn new() -> Self {
        CurrentBlock {
            id: 0,
            block: Self::random_block(),
            init_pos: BLOCK_POSITION,
        }
    }

    pub fn reset() -> Self { Self::new() }

    fn random_block() -> BlockType {
        let mut rng = rand::thread_rng();
        rng.gen()
    }
}

fn reset_current_block(
    mut current_block: ResMut<CurrentBlock>,
) {
    // debug!("reset current block");
    *current_block = CurrentBlock::reset();
}

pub struct BlockPlugin;

impl Plugin for BlockPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<MoveEvent>()
            .add_event::<RotationEvent>()
            .add_event::<CollisionEvent>()
            .add_event::<SpawnEvent>()
            .insert_resource(CurrentBlock::new())
            // .add_plugins(collision::CollisionPlugin)
            .add_plugins(movement::MovementPlugin)
            // .add_plugins(rotation::RotationPlugin)
            .add_plugins(spawn::SpawnPlugin)
            .add_systems(OnExit(AppState::Ingame), reset_current_block)
        ;
    }
}
