use bevy::prelude::*;
use rand::prelude::*;
use std::collections::VecDeque;

use super::block::BlockType;

pub mod prelude;

mod blockdata;

const RANDOMIZER_POOL_COUNT: usize = 35;
const RANDOMIZER_HISTORY_COUNT: usize = 4;

/// ブロックをランダムに生成するランダマイザ
/// - order: 直近に出た順序（重複排除付き）
/// - pool: ピース候補のプール（35個）
/// - history: 直近で出たブロック（同じブロックの連続防止）
/// - first: 最初だけ特別な動作をするフラグ
#[derive(Resource, Debug)]
pub struct BlockRandomizer {
    order: VecDeque<BlockType>,
    pool: [BlockType; RANDOMIZER_POOL_COUNT],
    history: VecDeque<BlockType>,
    first: bool,
}

impl BlockRandomizer {
    fn new() -> Self {
        // プールを7x5で埋める（公平性確保）
        let mut pool = [BlockType::TypeI; RANDOMIZER_POOL_COUNT];
        for (i, v) in pool.iter_mut().enumerate() {
            *v = BlockType::ALL[i % BlockType::ALL.len()];
        }

        // 一番初めに生成されるブロックは候補からランダム
        let first_piece = *BlockType::FIRST_CANDIDATES
            .choose(&mut thread_rng())
            .expect("FIRST_BLOCK_CANDINATES should not be empty");

        // 履歴を初期化（RGM3準拠：S, Z, S, 最初ブロック）
        let mut history = VecDeque::with_capacity(RANDOMIZER_HISTORY_COUNT);
        history.push_back(BlockType::TypeS);
        history.push_back(BlockType::TypeZ);
        history.push_back(BlockType::TypeS);
        history.push_back(first_piece);

        BlockRandomizer {
            order: VecDeque::new(),
            pool,
            history,
            first: true,
        }
    }
}

impl Iterator for BlockRandomizer {
    type Item = BlockType;

    // 次のブロックを返す（TGM3ランダマイザロジック準拠）
    fn next(&mut self) -> Option<Self::Item> {
        // 初回だけhistoryの末尾（first_piece）を返す
        if self.first {
            self.first = false;
            return self.history.back().copied();
        }

        let mut picked_piece = BlockType::TypeI;
        let mut idx = 0;
        let find_count = 6;
        // 最大6回まで「historyにないブロック」を探す
        for roll in 0..find_count {
            idx = thread_rng().gen_range(0..RANDOMIZER_POOL_COUNT);
            picked_piece = self.pool[idx];
            if !self.history.contains(&picked_piece) || roll == 5 {
                break;
            }
            // 既出順のブロックで置き換え
            if let Some(&first_order) = self.order.front() {
                self.pool[idx] = first_order;
            }
        }

        // orderを更新（重複を削除して追加）
        if let Some(pos) = self.order.iter().position(|&x| x == picked_piece) {
            self.order.remove(pos);
        }
        self.order.push_back(picked_piece);

        // poolを更新（orderの先頭で置き換え）
        if let Some(&first_order) = self.order.front() {
            self.pool[idx] = first_order;
        }

        // historyを更新（先頭を外して末尾に追加）
        if self.history.len() == RANDOMIZER_HISTORY_COUNT {
            self.history.pop_front();
        }
        self.history.push_back(picked_piece);

        Some(picked_piece)
    }
}

pub struct UtilsPlugin;

impl Plugin for UtilsPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(BlockRandomizer::new())
        ;
    }
}
