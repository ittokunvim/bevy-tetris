use bevy::prelude::*;

use crate::GRID_SIZE;
use crate::ingame::SpawnEvent;
use crate::ingame::utils::prelude::*;

/// ブロック生成イベントを処理する関数
/// `SpawnEvent`を受け取り、新しいブロックを生成してフィールドに配置します
pub fn block_spawn(
    mut events: EventReader<SpawnEvent>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut current_block: ResMut<CurrentBlocks>,
    mut nextblocks: ResMut<NextBlocks>,
    mut blockrandomizer: ResMut<BlockRandomizer>,
    query: Query<&Transform, With<Block>>,
) {
    info_once!("block_spawn");

    for event in events.read() {
        let blocktype = event.0.unwrap_or(nextblocks[1]);

        // 次ブロックデータを更新
        if event.0.is_none() {
            *nextblocks = nextblocks.update(blockrandomizer.next().unwrap());
        }

        // CurrentBlockをリセット
        *current_block = CurrentBlocks::new();

        // CurrentBlockのBlockTypeをNextBlockに紐付け
        current_block.blocktype = blocktype;

        // PlayerBlockを生成
        let shape = meshes.add(Rectangle::new(BLOCK_SIZE, BLOCK_SIZE));
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
}
