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
    Block,
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
    query: Query<&Transform, With<Block>>,
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
    let mut init_position = BLOCK_POSITION;

    for (index, value) in I_BLOCK[0].iter().enumerate() {
        // ブロックの値が0であればスキップ
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
            MeshMaterial2d(materials.add(I_COLOR)),
            Transform::from_xyz(position.x, position.y, position.z),
            PlayerBlock(*value),
        ));
    }
}

