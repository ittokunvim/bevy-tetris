use bevy::prelude::*;

use crate::GRID_SIZE;
use crate::utils::blockdata::*;
use super::{
    BLOCK_SIZE,
    BLOCK_POSITION,
    SpawnEvent,
    Block,
    BlockType,
};

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
    // このメソッドではブロックを生成する時のポジション(x,y)を返します。
    // ブロックデータは配列x4のusizex16からなり、usizex16から縦4マス横4マスの
    // ブロックの範囲とします。
    // そこからメソッドに渡されたIDを元にブロックデータの値を参照し、
    // 一致した箇所をxy軸に変換して返します。
    // 例
    // BlockType::TypeI.position(2) -> INITIAL_POSITION + Vec2::new(GRID_SIZE * 1, -GRID_SIZE * 1)
    fn position(&self, i: usize) -> Vec2 {
        let closure = |id: usize, block: [[usize; 16]; 4]| {
            let mut position = BLOCK_POSITION;

            for i in 0..block[0].len() {
                if id == block[0][i] {
                    // trace!("position: {}", position);
                    return position
                }
                position.x += GRID_SIZE;
                // movement y
                if i % 4 == 3 {
                    position.x = BLOCK_POSITION.x;
                    position.y -= GRID_SIZE;
                }
            }
            // id should be in the block[0]
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
    fn new(i: usize, block: BlockType) -> (Self, Sprite, Transform) {
        (
            Self,
            Sprite::from_color(block.color(), Vec2::ONE),
            Transform {
                translation: block.position(i).extend(1.0),
                scale: BLOCK_SIZE.extend(1.0),
                ..Default::default()
            },
        )
    }
}

fn setup(
    mut events: EventWriter<SpawnEvent>,
) {
    events.send_default();
}

fn spawn(
    mut events: EventReader<SpawnEvent>,
    mut commands: Commands,
) {
    if events.is_empty() { return }
    events.clear();
    // debug!("spawn");
    for i in 1..BLOCK_COUNT + 1 {
        // commands.spawn(Block::new(i, BlockType::TypeI));
        // commands.spawn(Block::new(i, BlockType::TypeJ));
        // commands.spawn(Block::new(i, BlockType::TypeL));
        commands.spawn(Block::new(i, BlockType::TypeO));
        // commands.spawn(Block::new(i, BlockType::TypeS));
        // commands.spawn(Block::new(i, BlockType::TypeT));
        // commands.spawn(Block::new(i, BlockType::TypeZ));
    }
}

pub struct SpawnPlugin;

impl Plugin for SpawnPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup)
            .add_systems(Update, spawn)
        ;
    }
}
