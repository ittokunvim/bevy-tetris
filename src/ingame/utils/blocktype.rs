use bevy::prelude::*;
use rand::{
    distributions::Standard,
    prelude::Distribution,
    Rng,
};

use crate::GRID_SIZE_HALF;
use super::prelude::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum BlockType {
    TypeI,
    TypeJ,
    TypeL,
    TypeO,
    TypeS,
    TypeT,
    TypeZ,
}

impl BlockType {
    /// 全てのテトリスブロック
    pub const ALL: [BlockType; 7] = [
        BlockType::TypeI,
        BlockType::TypeJ,
        BlockType::TypeL,
        BlockType::TypeO,
        BlockType::TypeS,
        BlockType::TypeT,
        BlockType::TypeZ,
    ];
    /// 最初に生成されるであろうブロック（THM3準拠）
    pub const FIRST_CANDIDATES: [BlockType; 4] = [
        BlockType::TypeI,
        BlockType::TypeJ,
        BlockType::TypeL,
        BlockType::TypeT,
    ];

    /// ブロックの形状データを取得するメソッド
    /// 各ブロックタイプに対応する4回転分の形状を持つ
    pub fn blockdata(&self) -> [[usize; 16]; 4] {
        match self {
            BlockType::TypeI => I_BLOCK,
            BlockType::TypeJ => J_BLOCK,
            BlockType::TypeL => L_BLOCK,
            BlockType::TypeO => O_BLOCK,
            BlockType::TypeS => S_BLOCK,
            BlockType::TypeT => T_BLOCK,
            BlockType::TypeZ => Z_BLOCK,
        }
    }

    /// ブロックに対応する色を取得するメソッド
    pub fn color(&self) -> Color {
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

    /// ブロックに対応する初期位置を返すメソッド
    pub fn calculate_position(&self, pos: Vec3) -> Vec2 {
        match self {
            BlockType::TypeI => Vec2::new(
                pos.x + GRID_SIZE_HALF * 0.5,
                pos.y - GRID_SIZE_HALF * 1.0,
            ),
            BlockType::TypeJ => Vec2::new(
                pos.x + GRID_SIZE_HALF * 1.0,
                pos.y - GRID_SIZE_HALF * 1.5,
            ),
            BlockType::TypeL => Vec2::new(
                pos.x + GRID_SIZE_HALF * 1.0,
                pos.y - GRID_SIZE_HALF * 1.5,
            ),
            BlockType::TypeO => Vec2::new(
                pos.x + GRID_SIZE_HALF * 0.5,
                pos.y - GRID_SIZE_HALF * 0.5,
            ),
            BlockType::TypeS => Vec2::new(
                pos.x + GRID_SIZE_HALF * 1.0,
                pos.y - GRID_SIZE_HALF * 0.5,
            ),
            BlockType::TypeT => Vec2::new(
                pos.x + GRID_SIZE_HALF * 1.0,
                pos.y - GRID_SIZE_HALF * 1.5,
            ),
            BlockType::TypeZ => Vec2::new(
                pos.x + GRID_SIZE_HALF * 1.0,
                pos.y - GRID_SIZE_HALF * 0.5,
            ),
        }
    }
}

/// ブロックの種類をランダムにサンプリングできるようにする
/// (rand::randomなどで利用可)
impl Distribution<BlockType> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> BlockType {
        let idx: usize = rng.gen_range(0..BlockType::ALL.len());
        BlockType::ALL[idx]
    }
}
