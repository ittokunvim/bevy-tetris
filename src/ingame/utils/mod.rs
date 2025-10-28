use bevy::prelude::*;

use crate::{
    GRID_SIZE,
    AppState,
};
use super::BlockSpawned;
use super::utils::{
    blockdata::*,
    blockrandomizer::BlockRandomizer,
    blocktype::BlockType,
    fielddata::*,
};

pub mod prelude;

mod blockdata;
mod blockrandomizer;
mod blocktype;
mod fielddata;

/// 移動、回転するブロックを識別するコンポーネント
/// 値には1~4に定義されているブロックのIDが格納される
#[derive(Component)]
pub struct PlayerBlock(pub usize);

/// 移動、回転しないブロックを識別するコンポーネント
/// ブロック削除時に使用される
#[derive(Component)]
pub struct Block;

/// ブロック削除時に用いるリソース
/// 値は[[usize; 10]; 24]で定義されており
/// フィールド内の各ブロック座標が0 or 1で格納されている
#[derive(Resource, Debug, Deref, DerefMut)]
pub struct BlockMap(pub [[usize; 10]; 24]);

impl BlockMap {
    /// 渡されたブロックの座標からブロックマップに値を代入するメソッド
    ///
    /// # Arguments
    /// * pos - ブロックの座標
    ///
    /// # Panics
    /// * 指定された座標が見つからない場合
    pub fn insert(&mut self, pos: Vec2) {
        let map = self.0;
        for y in 0..map.len() {
            for x in 0..map[0].len() {
                // ループから値に対応したXY座標を取得
                let current_pos = Vec2::new(
                    FIELD_LEFT_TOP.x + GRID_SIZE * x as f32, 
                    FIELD_LEFT_TOP.y + GRID_SIZE * 4.0 - GRID_SIZE * y as f32,
                );
                // そのXY座標と渡された値が一致したら、マップに値を追加
                if current_pos == pos {
                    return self.0[y][x] = 1;
                }
            }
        }
        // 値は必ず代入されなければならない
        panic!("pos no found: {}", pos);
    }

    /// 渡された削除するブロックの列のIDを参照して
    /// 消されるブロックをブロックマップに更新するメソッド
    ///
    /// # Arguments
    /// * index - 削除するブロックの列のID
    pub fn clearline(&mut self, index: usize) {
        let map = self.0;
        // 渡された値の行を全て0にする
        self.0[index] = [0; 10];
        // 0にした行から上の値を一段下にずらす
        for i in (1..=index).rev() {
            self.0[i] = map[i - 1];
        }
    }
}

/// 現在動かしているブロックを管理するリソース
/// idには[usize; 16]で定義されているindexが格納される
/// posには回転時に軸となるXYZ軸が定義される
#[derive(Resource)]
pub struct CurrentBlocks {
    pub blocktype: BlockType,
    pub blockid: usize,
    pub pos: Vec3,
}

impl CurrentBlocks {
    // リソースを初期化
    pub fn new() -> Self {
        CurrentBlocks {
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
        // ブロックIDは見つからなければならない
        panic!("id not found: {}", id);
    }
}

/// ホールドされたブロックを管理するリソース
/// - can_hold: ホールドが可能かどうか判定
/// - blocktype: ホールドされたブロックの形
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

/// 次に生成するブロックを管理するリソース
/// 値は[BlockType; NEXT_BLOCK_COUNT]で定義されており
/// 値にはランダムなブロックの形が格納されている
#[derive(Resource, Debug, Deref, DerefMut)]
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

/// ブロックが落下する速度を管理するリソース
/// タイマーが早くなればなるほどブロックが落下する速度も早くなる
#[derive(Resource, Deref, DerefMut)]
pub struct FallingTimer(pub Timer);

impl FallingTimer {
    pub fn new() -> Self {
        Self(Timer::from_seconds(BLOCK_FALL_SPEED, TimerMode::Repeating))
    }
}

/// ブロックを全て削除する関数
fn despawn(
    mut commands: Commands,
    query: Query<Entity, With<Block>>,
) {
    info_once!("despawn");

    for entity in &query {
        commands.entity(entity).despawn();
    }
}

/// リソースをセットアップする関数
fn setup(
    mut commands: Commands,
    mut _currentblock: ResMut<CurrentBlocks>,
    mut _blockmap: ResMut<BlockMap>,
    mut blockrandomizer: ResMut<BlockRandomizer>,
    mut _holdblocks: ResMut<HoldBlocks>,
    mut nextblocks: ResMut<NextBlocks>,
) {
    info_once!("setup");

    **nextblocks = std::array::from_fn(|_| blockrandomizer.next().unwrap());
    commands.trigger(BlockSpawned(Some(nextblocks[0])));
}

/// リソースをリセットする関数
fn reset(
    mut currentblock: ResMut<CurrentBlocks>,
    mut blockmap: ResMut<BlockMap>,
    mut blockrandomizer: ResMut<BlockRandomizer>,
    mut holdblocks: ResMut<HoldBlocks>,
    mut nextblocks: ResMut<NextBlocks>,
) {
    info_once!("reset");

    *currentblock = CurrentBlocks::new();
    *blockmap = BlockMap(BLOCK_MAP);
    *blockrandomizer = BlockRandomizer::new();
    *holdblocks = HoldBlocks::new();
    *nextblocks = NextBlocks::new();
}

pub struct UtilsPlugin;

impl Plugin for UtilsPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(CurrentBlocks::new())
            .insert_resource(BlockMap(BLOCK_MAP))
            .insert_resource(BlockRandomizer::new())
            .insert_resource(HoldBlocks::new())
            .insert_resource(NextBlocks::new())
            .insert_resource(FallingTimer::new())
            .add_systems(OnExit(AppState::Gameover), despawn)
            .add_systems(OnEnter(AppState::InGame), setup)
            .add_systems(OnExit(AppState::Gameover), reset)
         ;
    }
}
