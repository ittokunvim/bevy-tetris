use bevy::prelude::*;

use crate::{
    GRID_SIZE,
    SpawnEvent,
};
use crate::block::{
    BLOCK_POSITION,
    BLOCK_SIZE,
    RotationBlock,
    PlayerBlock,
};
use crate::blockdata::{
    I_BLOCK,
    I_COLOR,
};

/// ブロック生成イベントを処理する関数
/// `SpawnEvent`を受け取り、新しいブロックを生成してフィールドに配置します
///
pub fn block_spawn(
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

