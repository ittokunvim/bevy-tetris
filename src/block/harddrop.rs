use bevy::prelude::*;

use crate::{
    GRID_SIZE,
    FIELD_SIZE,
    FIELD_POSITION,
    HardDropEvent,
    FixEvent,
};

use super::{
    PlayerBlock,
    Block,
};

/// ブロックを一番下に固定する関数
/// 衝突判定が出るまで、ブロックを下に移動させて固定する
pub fn block_harddrop(
    mut harddrop_events: EventReader<HardDropEvent>,
    mut fix_events: EventWriter<FixEvent>,
    mut player_query: Query<&mut Transform, (With<PlayerBlock>, Without<Block>)>,
    block_query: Query<&Transform, With<Block>>,
) {
    info_once!("block_harddrop");

    // HardDropイベントが発火されなかったら何もしない
    if harddrop_events.is_empty() {
        return;
    }

    // イベントをクリア
    harddrop_events.clear();

    // 衝突判定が出るまで、ブロックを下に移動させる
    let mut collision = false;
    while !collision {
        // フィールドの衝突をチェック
        for transform in &mut player_query {
            let y = transform.translation.y;

            // もし下移動後のブロックのY座標がフィールドの底を超えていたら
            if y - GRID_SIZE < FIELD_POSITION.y - FIELD_SIZE.y / 2.0 {
                collision = true;
            }
        }

        // ブロックの衝突をチェック
        for player_transform in &mut player_query {
            let player_x = player_transform.translation.x;
            let player_y = player_transform.translation.y;

            for block_transform in &block_query {
                let block_x = block_transform.translation.x;
                let block_y = block_transform.translation.y;

                // もし下移動後のブロックの座標がブロックの座標が同じなら
                if player_x == block_x && player_y - GRID_SIZE == block_y {
                    collision = true;
                }
            }
        }

        // 現在動かしているブロックを移動
        if !collision {
            for mut transform in &mut player_query {
                transform.translation.y -= GRID_SIZE;
            }
        }
    }

    // ブロックを固定
    fix_events.send_default();
}
