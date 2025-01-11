use bevy::prelude::*;

use crate::GRID_SIZE;
use crate::utils::blockdata::*;

const SIZE: Vec2 = Vec2::splat(GRID_SIZE - 2.0);
const INITIAL_POSITION: Vec2 = Vec2::new(
    -1.0 * GRID_SIZE - GRID_SIZE / 2.0,
    10.0 * GRID_SIZE - GRID_SIZE / 2.0,
);
const FPS: f32 = 1.0;

#[derive(Component, Deref, DerefMut)]
struct FallingTimer(Timer);

#[derive(Component)]
#[require(Sprite, Transform, FallingTimer)]
struct Block;

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

impl Default for FallingTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(FPS, TimerMode::Repeating))
    }
}

impl BlockType {
    fn color(&self) -> Color {
        match self {
            BlockType::TypeI => I_COLOR,
            BlockType::TypeJ => J_COLOR,
            BlockType::TypeL => L_COLOR,
            BlockType::TypeO => O_COLOR,
            BlockType::TypeS => S_COLOR,
            BlockType::TypeT => T_COLOR,
            BlockType::TypeZ => Z_COLOR,
        }
    }

    fn position(&self, i: usize) -> Vec2 {
        let closure = |id: usize, block: [[usize; 16]; 4]| {
            let mut position = INITIAL_POSITION;

            for i in 0..block[0].len() {
                if id == block[0][i] {
                    // trace!("position: {}", position);
                    return position
                }
                position.x += GRID_SIZE;
                if i % 4 == 3 {
                    position.x = INITIAL_POSITION.x;
                    position.y -= GRID_SIZE;
                }
            }
            panic!("id: {} is not found", id);
        };

        match self {
            BlockType::TypeI => closure(i, I_BLOCK),
            BlockType::TypeJ => closure(i, J_BLOCK),
            BlockType::TypeL => closure(i, L_BLOCK),
            BlockType::TypeO => closure(i, O_BLOCK),
            BlockType::TypeS => closure(i, S_BLOCK),
            BlockType::TypeT => closure(i, T_BLOCK),
            BlockType::TypeZ => closure(i, Z_BLOCK),
        }
    }
}

impl Block {
    fn new(i: usize, block: BlockType) -> (Self, Sprite, Transform, FallingTimer) {
        (
            Self,
            Sprite::from_color(block.color(), Vec2::ONE),
            Transform {
                translation: block.position(i).extend(1.0),
                scale: SIZE.extend(1.0),
                ..Default::default()
            },
            FallingTimer::default(),
        )
    }
}

fn setup(
    mut commands: Commands,
) {
    for i in 1..BLOCK_COUNT + 1 {
        commands.spawn(Block::new(i, BlockType::TypeI));
        // commands.spawn(Block::new(i, BlockType::TypeJ));
        // commands.spawn(Block::new(i, BlockType::TypeL));
        // commands.spawn(Block::new(i, BlockType::TypeO));
        // commands.spawn(Block::new(i, BlockType::TypeS));
        // commands.spawn(Block::new(i, BlockType::TypeT));
        // commands.spawn(Block::new(i, BlockType::TypeZ));
    }
}

fn falling(
    mut query: Query<(&mut FallingTimer, &mut Transform), With<Block>>,
    time: Res<Time>,
) {
    for (mut timer, mut transform) in &mut query {
        timer.tick(time.delta());
        if !timer.just_finished() { continue }
        timer.reset();
        // debug!("movement");
        transform.translation.y -= GRID_SIZE;
    }
}

pub struct BlocksPlugin;

impl Plugin for BlocksPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup)
            .add_systems(Update, falling)
        ;
    }
}
