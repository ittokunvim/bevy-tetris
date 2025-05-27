use bevy::prelude::*;

use crate::GRID_SIZE;
use crate::ingame::SpawnEvent;
use crate::ingame::utils::prelude::*;

/// ゲーム開始時に実行される関数
/// プレイヤーが動かすブロックを生成する
pub fn setup_spawn(mut events: EventWriter<SpawnEvent>) {
    info_once!("setup_spawn");

    events.send_default();
}

/// ブロック生成イベントを処理する関数
/// `SpawnEvent`を受け取り、新しいブロックを生成してフィールドに配置します
pub fn block_spawn(
    mut events: EventReader<SpawnEvent>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut current_block: ResMut<CurrentBlock>,
    nextblocks: Res<NextBlocks>,
    query: Query<&Transform, With<Block>>,
) {
    info_once!("block_spawn");

    // イベントをチェック
    if events.is_empty() {
        return;
    }

    // イベントをクリア
    events.clear();

    // CurrentBlockをリセット
    *current_block = CurrentBlock::new();

    // CurrentBlockのBlockTypeをNextBlockに紐付け
    current_block.blocktype = *nextblocks.0.first().unwrap();

    // PlayerBlockを生成
    let shape = meshes.add(Rectangle::new(BLOCK_SIZE, BLOCK_SIZE));
    let blocktype = &current_block.blocktype;
    let mut init_position = BLOCK_POSITION;

    for (index, value) in blocktype.blockdata()[0].iter().enumerate() {
        // ブロックデータの値が0であればスキップ
        if *value == 0 {
            continue;
        }

        // ブロックの位置を計算
        let mut position = Vec3::new(
            init_position.x + GRID_SIZE * ((index % 4) as f32),
            init_position.y - GRID_SIZE * ((index / 4) as f32),
            init_position.z,
        );

        // ブロックが同士が被らないように位置を計算
        for transform in &query {
            if position == transform.translation {
                position.y += GRID_SIZE;
                init_position.y += GRID_SIZE;
            }
        }

        // PlayerBlockを生成
        commands.spawn((
            Mesh2d(shape.clone()),
            MeshMaterial2d(materials.add(blocktype.color())),
            Transform::from_xyz(position.x, position.y, position.z),
            PlayerBlock(*value),
        ));
    }
}
