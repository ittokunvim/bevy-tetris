use bevy::prelude::*;
use rand::{
    distributions::Standard,
    prelude::Distribution,
    Rng,
};

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

    // ブロックの形状データを取得するメソッド
    // 各ブロックタイプに対応する4回転分の形状を持つ
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

    // ブロックに対応する色を取得するメソッド
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
}

/// ブロックの種類をランダムにサンプリングできるようにする
/// (rand::randomなどで利用可)
impl Distribution<BlockType> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> BlockType {
        let idx: usize = rng.gen_range(0..BlockType::ALL.len());
        BlockType::ALL[idx]
    }
}
