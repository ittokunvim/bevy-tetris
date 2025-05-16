use bevy::prelude::*;
use rand::{
    distributions::Standard,
    prelude::Distribution,
    Rng,
};

use crate::{
    GRID_SIZE,
    FIELD_SIZE,
    FIELD_POSITION,
    SpawnEvent,
    FixEvent,
    AppState,
};
use crate::blockdata::*;

mod clear;
mod gizmos;
mod harddrop;
mod movement;
mod rotation;
mod spawn;

const MAX_BLOCK_COUNT: usize = 4;
const MAX_COLLISION_COUNT: usize = 3;
const BLOCK_SIZE: f32 = GRID_SIZE - 1.0;
const BLOCK_POSITION: Vec3 = Vec3::new(
    FIELD_POSITION.x + GRID_SIZE / 2.0 - GRID_SIZE * 2.0,
    FIELD_POSITION.y + GRID_SIZE / 2.0 + FIELD_SIZE.y / 2.0 - GRID_SIZE * 1.0,
    10.0,
);
const FIELD_LEFT_TOP: Vec2 = Vec2::new(
    FIELD_POSITION.x - FIELD_SIZE.x / 2.0 + GRID_SIZE / 2.0, 
    FIELD_POSITION.y + FIELD_SIZE.y / 2.0 - GRID_SIZE / 2.0,
);
const NEXT_BLOCK_COUNT: usize = 4;

/// ブロック回転時に用いるリソース
///
/// idには[usize; 16]で定義されているindexが格納される
/// posには回転時に軸となるXYZ軸が定義される
#[derive(Resource)]
struct CurrentBlock {
    blocktype: BlockType,
    blockid: usize,
    pos: Vec3,
}

/// ブロック削除時に用いるリソース
///
/// 値は[[usize; 10]; 24]で定義されており
/// フィールド内の各ブロック座標が0 or 1で格納されている
#[derive(Resource)]
struct BlockMap([[usize; 10]; 24]);

/// 次に生成するブロックの表示に用いるリソース
///
/// 値は[BlockType; 3]で定義されており
/// ブロックの形に関する値が格納されている
#[derive(Resource, Debug)]
pub struct NextBlocks(pub [BlockType; NEXT_BLOCK_COUNT]);

#[derive(Copy, Clone, Debug)]
pub enum BlockType {
    TypeI,
    TypeJ,
    TypeL,
    TypeO,
    TypeS,
    TypeT,
    TypeZ,
}

/// 移動、回転するブロックを識別するコンポーネント
///
/// 値には1~4に定義されているブロックのIDが格納される
#[derive(Component)]
struct PlayerBlock(usize);

/// 移動、回転しないブロックを識別するコンポーネント
///
/// ブロック削除時に使用される
#[derive(Component)]
struct Block;

impl CurrentBlock {
    // リソースを初期化
    fn new() -> Self {
        CurrentBlock {
            blocktype: BlockType::random(),
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
    fn position(&self, id: usize) -> Vec3 {
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
    fn insert(&self, pos: Vec2) -> [[usize; 10]; 24] {
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
    fn clearline(&self, index: usize) -> [[usize; 10]; 24] {
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

impl Distribution<BlockType> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> BlockType {
        // 配列を使用してインデックスでブロックを選ぶ
        const BLOCK_TYPES: [BlockType; 7] = [
            BlockType::TypeI,
            BlockType::TypeJ,
            BlockType::TypeL,
            BlockType::TypeO,
            BlockType::TypeS,
            BlockType::TypeT,
            BlockType::TypeZ,
        ];

        let index: usize = rng.gen_range(0..BLOCK_TYPES.len());
        BLOCK_TYPES[index]
    }
}

impl BlockType {
    // `BlockType`をランダムに生成するヘルパーメソッド
    fn random() -> Self {
        let mut rng = rand::thread_rng();
        rng.gen()
    }

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

impl NextBlocks {
    fn new() -> Self {
        let blocktypes = std::array::from_fn(|_| BlockType::random());

        Self(blocktypes)
    }

    pub fn update(&self) -> Self {
        let mut blocktypes = self.0;

        // 配列の長さを保証
        assert!(blocktypes.len() == NEXT_BLOCK_COUNT, "Unexpected blocktypes length");

        // 配列を1つ左にシフト
        blocktypes.copy_within(1.., 0);

        // 最後の要素をランダム値で更新
        blocktypes[NEXT_BLOCK_COUNT - 1] = BlockType::random();

        Self(blocktypes)
    }
}

fn setup(mut events: EventWriter<SpawnEvent>) {
    info_once!("setup");

    events.send_default();
}

/// ゲームオーバーを管理する関数
/// `FixEvent`を受け取り、固定されたブロックから
/// ゲームオーバーになるかどうかチェックします
///
fn gameover(
    mut events: EventReader<FixEvent>,
    mut next_state: ResMut<NextState<AppState>>,
    query: Query<&Transform, With<PlayerBlock>>,
) {
    info_once!("gameover");

    // イベントをチェック
    if events.is_empty() {
        return;
    }

    // イベントをクリア
    events.clear();

    // ゲームオーバーかどうか判定する
    for transform in &query {
        let pos = transform.translation;
        if pos.y >= FIELD_LEFT_TOP.y {
            if pos.x == FIELD_LEFT_TOP.x + GRID_SIZE * 5.0
            || pos.x == FIELD_LEFT_TOP.x + GRID_SIZE * 6.0 {
                next_state.set(AppState::Gameover);
                return;
            }
        }
    }
}

fn despawn(
    mut commands: Commands,
    query: Query<Entity, With<Block>>,
) {
    info_once!("despawn");

    for entity in &query {
        commands.entity(entity).despawn();
    }
}

fn reset(
    mut rotation_block: ResMut<CurrentBlock>,
    mut block_map: ResMut<BlockMap>,
) {
    info_once!("reset");

    *rotation_block = CurrentBlock::new();
    *block_map = BlockMap(BLOCK_MAP);
}

pub struct BlockPlugin;

impl Plugin for BlockPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(CurrentBlock::new())
            .insert_resource(BlockMap(BLOCK_MAP))
            .insert_resource(NextBlocks::new())
            .add_systems(OnEnter(AppState::InGame), setup)
            .add_systems(Update, (
                spawn::block_spawn,
                movement::block_falling,
                rotation::block_rotation,
                movement::block_movement,
                harddrop::block_harddrop,
                gizmos::draw_gizmos_block,
                gameover,
                clear::block_clear,
            ).chain().run_if(in_state(AppState::InGame)))
            .add_systems(OnExit(AppState::Gameover), despawn)
            .add_systems(OnExit(AppState::Gameover), reset)
        ;
    }
}
