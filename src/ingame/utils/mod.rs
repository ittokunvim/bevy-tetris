use bevy::{
    prelude::*,
    time::Stopwatch,
};

use crate::GRID_SIZE;
use super::utils::{
    blockdata::*,
    blockrandomizer::BlockRandomizer,
    blocktype::BlockType,
};

pub mod prelude;

mod blockdata;
mod blockrandomizer;
mod blocktype;

/// 移動、回転するブロックを識別するコンポーネント
///
/// 値には1~4に定義されているブロックのIDが格納される
#[derive(Component)]
pub struct PlayerBlock(pub usize);

/// 移動、回転しないブロックを識別するコンポーネント
///
/// ブロック削除時に使用される
#[derive(Component)]
pub struct Block;

/// ブロック削除時に用いるリソース
///
/// 値は[[usize; 10]; 24]で定義されており
/// フィールド内の各ブロック座標が0 or 1で格納されている
#[derive(Resource)]
pub struct BlockMap(pub [[usize; 10]; 24]);

impl BlockMap {
    /// 渡されたブロックの座標からブロックマップに値を代入し
    /// そのブロックマップを返すメソッド
    ///
    /// # Arguments
    /// * pos - ブロックの座標
    ///
    /// # Returns
    /// * [[usize; 10]; 24] - 更新されたブロックマップ
    ///
    /// # Panics
    /// * 指定された座標が見つからない場合
    pub fn insert(&self, pos: Vec2) -> [[usize; 10]; 24] {
        let mut block_map = self.0;
        // ブロック座標にブロックマップを追加
        for y in 0..block_map.len() {
            for x in 0..block_map[0].len() {
                let current_pos = Vec2::new(
                    FIELD_LEFT_TOP.x + GRID_SIZE * x as f32, 
                    FIELD_LEFT_TOP.y + GRID_SIZE * 4.0 - GRID_SIZE * y as f32,
                );
                if current_pos == pos {
                    block_map[y][x] = 1;
                    return block_map
                }
            }
        }
        panic!("pos no found: {}", pos);
    }

    /// 渡された削除するブロックの列のIDを参照して
    /// 消されるブロックをブロックマップに更新し
    /// ブロック削除後のブロックマップを返すメソッド
    ///
    /// # Arguments
    /// * index - 削除するブロックの列のID
    ///
    /// # Returns
    /// * [[usize; 10]; 24] - 更新されたブロックマップ
    pub fn clearline(&self, index: usize) -> [[usize; 10]; 24] {
        let mut block_map = self.0;
        // clear index line
        block_map[index] = [0; 10];
        // shift down one by one
        for i in (1..=index).rev() {
            block_map[i] = block_map[i - 1];
        }
        // clear top line
        block_map[0] = [0; 10];
        block_map
    }
}

/// ブロック回転時に用いるリソース
///
/// idには[usize; 16]で定義されているindexが格納される
/// posには回転時に軸となるXYZ軸が定義される
#[derive(Resource)]
pub struct CurrentBlock {
    pub blocktype: BlockType,
    pub blockid: usize,
    pub pos: Vec3,
}

impl CurrentBlock {
    // リソースを初期化
    pub fn new() -> Self {
        CurrentBlock {
            blocktype: BlockType::TypeI,
            blockid: 0,
            pos: BLOCK_POSITION,
        }
    }
    /// 渡されたブロックIDの回転後のブロックの位置を返すメソッド
    ///
    /// # Arguments
    /// * id - 回転後のブロックの位置を取得するためのブロックID
    ///
    /// # Returns
    /// * Vec3 - 回転後のブロックの位置
    ///
    /// # Panics
    /// * idが見つからない場合
    pub fn position(&self, id: usize) -> Vec3 {
        let blockdata = self.blocktype.blockdata();

        // ブロックIDが有効範囲内かチェック
        assert!(self.blockid < blockdata.len());
        // 回転後のブロックの位置を見つける
        for (index, value) in blockdata[self.blockid].iter().enumerate() {
            if id == *value {
                // ブロックの新しい位置を計算して返す
                let (x, y, z) = (
                    self.pos.x + GRID_SIZE * ((index % 4) as f32),
                    self.pos.y - GRID_SIZE * ((index / 4) as f32),
                    self.pos.z,
                );
                return Vec3::new(x, y, z);
            }
        }
        // ブロックIDが見つからなかったらパニック
        panic!("id not found: {}", id);
    }
}

#[derive(Resource)]
pub struct HoldBlocks {
    pub can_hold: bool,
    pub blocktype: Option<BlockType>,
}

impl HoldBlocks {
    pub fn new() -> Self {
        Self {
            can_hold: true,
            blocktype: None,
        }
    }
}

/// 次に生成するブロックの表示に用いるリソース
///
/// 値は[BlockType; 3]で定義されており
/// ブロックの形に関する値が格納されている
#[derive(Resource, Debug)]
pub struct NextBlocks(pub [BlockType; NEXT_BLOCK_COUNT]);

impl NextBlocks {
    pub fn new() -> Self {
        let blocktypes = std::array::from_fn(|_| BlockType::TypeI);

        Self(blocktypes)
    }

    pub fn update(&self, blocktype: BlockType) -> Self {
        let mut blocktypes = self.0;

        // 配列の長さを保証
        assert!(blocktypes.len() == NEXT_BLOCK_COUNT, "Unexpected blocktypes length");

        // 配列を1つ左にシフト
        blocktypes.copy_within(1.., 0);

        // 最後の要素をランダム値で更新
        blocktypes[NEXT_BLOCK_COUNT - 1] = blocktype;

        Self(blocktypes)
    }
}

#[derive(Resource, Deref, DerefMut)]
pub struct FallingTimer(pub Timer);

impl FallingTimer {
    pub fn new() -> Self {
        Self(Timer::from_seconds(BLOCK_FALL_SPEED, TimerMode::Repeating))
    }
}

#[derive(Resource, Deref, DerefMut)]
pub struct MoveLeftTimer(pub Stopwatch);

#[derive(Resource, Deref, DerefMut)]
pub struct MoveRightTimer(pub Stopwatch);

#[derive(Resource, Deref, DerefMut)]
pub struct MoveBottomTimer(pub Stopwatch);

pub struct UtilsPlugin;

impl Plugin for UtilsPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(CurrentBlock::new())
            .insert_resource(BlockMap(BLOCK_MAP))
            .insert_resource(BlockRandomizer::new())
            .insert_resource(HoldBlocks::new())
            .insert_resource(NextBlocks::new())
            .insert_resource(FallingTimer::new())
            .insert_resource(MoveLeftTimer(Stopwatch::new()))
            .insert_resource(MoveRightTimer(Stopwatch::new()))
            .insert_resource(MoveBottomTimer(Stopwatch::new()))

         ;
    }
}
