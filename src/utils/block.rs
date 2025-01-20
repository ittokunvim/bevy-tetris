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
use crate::utils::blockdata::*;

pub const BLOCK_SPEED: f32 = 0.2;
const BLOCK_SIZE: Vec2 = Vec2::splat(GRID_SIZE - 2.0);
const BLOCK_POSITION: Vec2 = Vec2::new(
    -1.0 * GRID_SIZE - GRID_SIZE / 2.0,
    10.0 * GRID_SIZE - GRID_SIZE / 2.0,
);

#[derive(Resource)]
pub struct CurrentBlock {
    pub id: usize,
    pub block: BlockType,
    pub init_pos: Vec2,
}

#[derive(Component)]
pub struct PlayerBlock(pub usize);

#[derive(Component)]
#[require(Sprite, Transform)]
pub struct Block;

#[allow(dead_code)]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum BlockType {
    TypeI,
    TypeJ,
    TypeL,
    TypeO,
    TypeS,
    TypeT,
    TypeZ,
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
    pub fn position(
        &self,
        init_pos: Vec2,
        blockdata_id: usize,
        block_id: usize,
    ) -> Vec2 {
        let closure = |block: [[usize; 16]; 4]| {
            let mut position = init_pos;

            for i in 0..block[blockdata_id].len() {
                if block_id == block[blockdata_id][i] {
                    // trace!("position: {}", position);
                    return position
                }
                position.x += GRID_SIZE;
                // movement y
                if i % 4 == 3 {
                    position.x = init_pos.x;
                    position.y -= GRID_SIZE;
                }
            }
            // id should be in the block[0]
            panic!("id: {} is not found", block_id);
        };

        match self {
            BlockType::TypeI => closure(I_BLOCK),
            BlockType::TypeJ => closure(J_BLOCK),
            BlockType::TypeL => closure(L_BLOCK),
            BlockType::TypeO => closure(O_BLOCK),
            BlockType::TypeS => closure(S_BLOCK),
            BlockType::TypeT => closure(T_BLOCK),
            BlockType::TypeZ => closure(Z_BLOCK),
        }
    }
}

impl Block {
    pub fn new(
        blockdata_id: usize,
        block_id: usize,
        block: BlockType,
    ) -> (Self, Sprite, Transform, PlayerBlock) {
        (
            Self,
            Sprite::from_color(block.color(), Vec2::ONE),
            Transform {
                translation: block.position(BLOCK_POSITION, blockdata_id, block_id).extend(1.0),
                scale: BLOCK_SIZE.extend(1.0),
                ..Default::default()
            },
            PlayerBlock(block_id),
        )
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
            .insert_resource(CurrentBlock::new())
            .add_systems(OnExit(AppState::Ingame), reset_current_block)
        ;
    }
}
