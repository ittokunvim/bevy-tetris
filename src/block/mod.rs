use bevy::prelude::*;

use crate::{
    GRID_SIZE,
    FIELD_SIZE,
    FIELD_POSITION,
    MoveEvent,
    RotationEvent,
    SpawnEvent,
    FixEvent,
    Direction,
    FallingTimer,
};
use crate::blockdata::{
    BLOCK_MAP,
    I_BLOCK,
    I_COLOR,
};

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

/// ブロック回転時に用いるリソース
///
/// idには[usize; 16]で定義されているindexが格納される
/// posには回転時に軸となるXYZ軸が定義される
#[derive(Resource)]
struct RotationBlock {
    id: usize,
    pos: Vec3,
}

/// ブロック削除時に用いるリソース
///
/// 値は[[usize; 10]; 20]で定義されており
/// フィールド内の各ブロック座標が0 or 1で格納されている
#[derive(Resource)]
struct BlockMap([[usize; 10]; 20]);

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

impl RotationBlock {
    // リソースを初期化
    fn new() -> Self {
        RotationBlock {
            id: 0,
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
        // ブロックIDが有効範囲内かチェック
        assert!(self.id < I_BLOCK.len());
        // 回転後のブロックの位置を見つける
        for (index, value) in I_BLOCK[self.id].iter().enumerate() {
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
    /// * [[usize; 10]; 20] - 更新されたブロックマップ
    ///
    /// # Panics
    /// * 指定された座標が見つからない場合
    fn insert(&self, pos: Vec2) -> [[usize; 10]; 20] {
        let mut block_map = self.0;
        // ブロック座標にブロックマップを追加
        for y in 0..block_map.len() {
            for x in 0..block_map[0].len() {
                let current_pos = Vec2::new(
                    FIELD_LEFT_TOP.x + GRID_SIZE * x as f32, 
                    FIELD_LEFT_TOP.y - GRID_SIZE * y as f32,
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
    /// * [[usize; 10]; 20] - 更新されたブロックマップ
    fn clearline(&self, index: usize) -> [[usize; 10]; 20] {
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

fn setup(mut events: EventWriter<SpawnEvent>) { events.send_default(); }

/// ブロック生成イベントを処理する関数
/// `SpawnEvent`を受け取り、新しいブロックを生成してフィールドに配置します
///
fn block_spawn(
    mut events: EventReader<SpawnEvent>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut rotation_block: ResMut<RotationBlock>,
) {
    // イベントをチェック
    if events.is_empty() {
        return;
    }

    // イベントをクリア
    events.clear();

    // RotationBlockをリセット
    *rotation_block = RotationBlock::new();

    // PlayerBlockを生成
    let shape = meshes.add(Rectangle::new(BLOCK_SIZE, BLOCK_SIZE));

    for (index, value) in I_BLOCK[0].iter().enumerate() {
        // ブロックの位置を計算
        let (x, y, z) = (
            BLOCK_POSITION.x + GRID_SIZE * ((index % 4) as f32),
            BLOCK_POSITION.y - GRID_SIZE * ((index / 4) as f32),
            BLOCK_POSITION.z,
        );

        // ブロックの値が0であればスキップ
        if *value == 0 {
            continue;
        }

        // PlayerBlockを生成
        commands.spawn((
            Mesh2d(shape.clone()),
            MeshMaterial2d(materials.add(I_COLOR)),
            Transform::from_xyz(x, y, z),
            PlayerBlock(*value),
        ));
    }
}

/// ブロックの落下を管理する関数
/// `FallingTimer`を使用して一定間隔でブロックを下に移動させる
///
fn block_falling(
    mut timer: ResMut<FallingTimer>,
    mut events: EventWriter<MoveEvent>,
    time: Res<Time>,
) {
    // タイマーを進める
    timer.tick(time.delta());

    // タイマーが終わったかチェック
    if !timer.just_finished() {
        return;
    }

    // ブロックを下に移動させるイベントを送信
    events.send(MoveEvent(Direction::Bottom));
}

/// ブロックの移動を管理する関数
/// `MoveEvent`を受け取り、ブロックの位置を更新し、
/// 必要に応じてブロックを固定する
///
fn block_movement(
    mut move_events: EventReader<MoveEvent>,
    mut fix_events: EventWriter<FixEvent>,
    mut player_query: Query<&mut Transform, (With<PlayerBlock>, Without<Block>)>,
    mut rotation_block: ResMut<RotationBlock>,
    block_query: Query<&Transform, With<Block>>,
) {
    for event in move_events.read() {
        let direction = event.0;

        // フィールドの衝突をチェック
        for player_transform in &mut player_query {
            let player_x = player_transform.translation.x;
            let player_y = player_transform.translation.y;

            match direction {
                Direction::Left => {
                    if player_x - GRID_SIZE < FIELD_POSITION.x - FIELD_SIZE.x / 2.0 {
                        return;
                    }
                }
                Direction::Right => {
                    if player_x + GRID_SIZE > FIELD_POSITION.x + FIELD_SIZE.x / 2.0 {
                        return;
                    }
                }
                Direction::Bottom => {
                    if player_y - GRID_SIZE < FIELD_POSITION.y - FIELD_SIZE.y / 2.0 {
                        // ブロックがそこに達した場合、ブロックを固定
                        fix_events.send_default();
                        return;
                    }
                }
            }

            // ブロックの衝突をチェック
            for block_transform in &block_query {
                let block_x = block_transform.translation.x;
                let block_y = block_transform.translation.y;

                match direction {
                    Direction::Left => {
                        if player_x - GRID_SIZE == block_x && player_y == block_y {
                            return;
                        }
                    }
                    Direction::Right => {
                        if player_x + GRID_SIZE == block_x && player_y == block_y {
                            return;
                        }
                    }
                    Direction::Bottom => {
                        if player_x == block_x && player_y - GRID_SIZE == block_y {
                            // ブロックが底に達した場合、ブロックを固定
                            fix_events.send_default();
                            return;
                        }
                    }
                }
            }
        }

        // 現在のブロック位置を更新
        match direction {
            Direction::Left   => rotation_block.pos.x -= GRID_SIZE,
            Direction::Right  => rotation_block.pos.x += GRID_SIZE,
            Direction::Bottom => rotation_block.pos.y -= GRID_SIZE,
        }
        // ブロックを移動
        for mut transform in &mut player_query {
            match direction {
                Direction::Left   => transform.translation.x -= GRID_SIZE,
                Direction::Right  => transform.translation.x += GRID_SIZE,
                Direction::Bottom => transform.translation.y -= GRID_SIZE,
            }
        }
    }
}

/// ブロックの回転を管理する関数
/// `RotationEvent`を受け取り、ブロックの位置を更新し、
/// 必要に応じてブロックの衝突を処理します
///
fn block_rotation(
    mut events: EventReader<RotationEvent>,
    mut timer: ResMut<FallingTimer>,
    mut player_query: Query<(&PlayerBlock, &mut Transform), (With<PlayerBlock>, Without<Block>)>,
    mut rotation_block: ResMut<RotationBlock>,
    block_query: Query<&Transform, With<Block>>,
) {
    for event in events.read() {
        let direction = event.0;
        let mut count = 0;
        let mut collision_x = 0.0;
        let mut collision_y = 0.0;

        // タイマーをリセット
        timer.reset();

        // 現在のブロックIDを更新
        rotation_block.id = match direction {
            Direction::Right => (rotation_block.id + 1) % MAX_BLOCK_COUNT,
            Direction::Left  => (rotation_block.id + MAX_BLOCK_COUNT - 1) % MAX_BLOCK_COUNT,
            _ => rotation_block.id,
        };

        // 衝突をチェック
        for (player, mut _player_transform) in &mut player_query {
            while count < MAX_COLLISION_COUNT {
                // 回転時のブロックの位置を取得
                let position = rotation_block.position(player.0);

                // フィールド左側の衝突判定
                if position.x < FIELD_POSITION.x - FIELD_SIZE.x / 2.0 {
                    rotation_block.pos.x += GRID_SIZE;
                    collision_x += GRID_SIZE;
                    count += 1;
                }
                // フィールド右側の衝突判定
                else if position.x > FIELD_POSITION.x + FIELD_SIZE.x / 2.0 {
                    rotation_block.pos.x -= GRID_SIZE;
                    collision_x -= GRID_SIZE;
                    count += 1;
                }
                // フィールド下側の衝突判定
                else if position.y < FIELD_POSITION.y - FIELD_SIZE.y / 2.0 {
                    rotation_block.pos.y += GRID_SIZE;
                    collision_y += GRID_SIZE;
                    count += 1;
                }
                // ブロック同士の衝突判定
                else if block_query.iter().any(|block_transform|
                    position == block_transform.translation
                ) {
                    rotation_block.pos.y += GRID_SIZE;
                    collision_y += GRID_SIZE;
                    count += 1;
                }
                // 衝突がなければループを抜ける
                else { break; }
            }
        }

        // もし衝突判定が規定回数以上あった場合、回転を行わない
        if count >= MAX_COLLISION_COUNT {
            // 現在のブロックIDをリセット
            rotation_block.id = match direction {
                Direction::Right => (rotation_block.id + MAX_BLOCK_COUNT - 1) % MAX_BLOCK_COUNT,
                Direction::Left  => (rotation_block.id + 1) % MAX_BLOCK_COUNT,
                _ => rotation_block.id,
            };
            // 現在のブロック位置をリセット
            rotation_block.pos.x -= collision_x;
            rotation_block.pos.y -= collision_y;
            return;
        }
        // ブロックを回転させる
        for (player, mut player_transform) in &mut player_query {
            player_transform.translation = rotation_block.position(player.0);
        }
    }
}

/// ブロックの削除を管理する関数
/// `FixEvent`を受け取り、プレイヤーブロックを固定ブロックに変換し、
/// ブロックマップを更新して、ラインが揃った場合にブロックを削除します。
///
fn block_clear(
    mut fix_events: EventReader<FixEvent>,
    mut commands: Commands,
    mut player_query: Query<(Entity, &mut Transform), (With<PlayerBlock>, Without<Block>)>,
    mut block_query: Query<(Entity, &mut Transform), (With<Block>, Without<PlayerBlock>)>,
    mut block_map: ResMut<BlockMap>,
    mut spawn_events: EventWriter<SpawnEvent>,
) {
    // イベントをチェック
    if fix_events.is_empty() {
        return;
    }

    // イベントをクリア
    fix_events.clear();

    // PlayerBlockをBlockに変換
    for (player_entity, player_transform) in &player_query {
        commands.entity(player_entity).remove::<PlayerBlock>();
        commands.entity(player_entity).insert(Block);

        // BlockMapを更新
        let pos = player_transform.translation.truncate();
        block_map.0 = block_map.insert(pos);
    }

    let map = block_map.0;

    // ブロックを削除
    for (index, row) in map.iter().enumerate() {
        if *row == [1; 10] {
            let y = FIELD_LEFT_TOP.y - GRID_SIZE * index as f32;
            block_map.0 = block_map.clearline(index);

            // プレイヤーブロックをチェック
            for (player_entity, mut player_transform) in &mut player_query {
                if player_transform.translation.y == y {
                    commands.entity(player_entity).despawn();
                }
                if player_transform.translation.y > y {
                    player_transform.translation.y -= GRID_SIZE;
                }
            }

            // 固定ブロックをチェック
            for (block_entity, mut block_transform) in &mut block_query {
                if block_transform.translation.y == y {
                    commands.entity(block_entity).despawn();
                }
                if block_transform.translation.y > y {
                    block_transform.translation.y -= GRID_SIZE;
                }
            }
        }
    }

    // ブロックを生成するイベントを送信
    spawn_events.send_default();
}

pub struct BlockPlugin;

impl Plugin for BlockPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(RotationBlock::new())
            .insert_resource(BlockMap(BLOCK_MAP))
            .add_systems(Startup, setup)
            .add_systems(Update, (
                block_spawn,
                block_falling,
                block_rotation,
                block_movement,
                block_clear,
            ).chain())
        ;
    }
}
