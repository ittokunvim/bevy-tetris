use bevy::prelude::*;

use crate::{
    GRID_SIZE,
    SpawnEvent,
    FixEvent,
};
use crate::block::{
    FIELD_LEFT_TOP,
    BlockMap,
    PlayerBlock,
    Block,
};

/// ブロックの削除を管理する関数
/// `FixEvent`を受け取り、プレイヤーブロックを固定ブロックに変換し、
/// ブロックマップを更新して、ラインが揃った場合にブロックを削除します。
///
pub fn block_clear(
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

