use bevy::prelude::*;

use crate::{
    GRID_SIZE,
    SpawnEvent,
    HoldEvent,
};

use super::{
    BLOCK_SIZE,
    BLOCK_POSITION,
    CurrentBlock,
    HoldBlocks,
    PlayerBlock,
    Block,
};

pub fn block_hold(
    mut hold_events: EventReader<HoldEvent>,
    mut spawn_events: EventWriter<SpawnEvent>,
    mut commands: Commands,
    mut holdblocks: ResMut<HoldBlocks>,
    mut currentblock: ResMut<CurrentBlock>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    query: Query<&Transform, With<Block>>,
) {
    info_once!("block_hold");

    for event in hold_events.read() {
        let blocktype = event.0;

        // まだ何もホールドしていない場合は、ホールド登録＆新規ブロック生成イベント発火
        if holdblocks.blocktype.is_none() {
            holdblocks.blocktype = Some(blocktype);
            spawn_events.send_default();
            return;
        }

        // 一時的にCurrentBlockを初期化し、ホールドブロックを入れ替え
        *currentblock = CurrentBlock::new();
        currentblock.blocktype = holdblocks.blocktype.unwrap();
        holdblocks.blocktype = Some(blocktype);

        // プレイヤーブロックを生成
        let shape = meshes.add(Rectangle::new(BLOCK_SIZE, BLOCK_SIZE));
        let blocktype = currentblock.blocktype;
        let mut init_position = BLOCK_POSITION;

        for (index, &cell) in blocktype.blockdata()[0].iter().enumerate() {
            // ブロックの値が0であればスキップ
            if cell == 0 {
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

            // プレイヤーブロックを生成
            commands.spawn((
                Mesh2d(shape.clone()),
                MeshMaterial2d(materials.add(blocktype.color())),
                Transform::from_xyz(position.x, position.y, position.z),
                PlayerBlock(cell),
            ));
        }
    }
}
