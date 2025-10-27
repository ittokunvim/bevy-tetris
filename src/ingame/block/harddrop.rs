use bevy::prelude::*;

use crate::GRID_SIZE;
use crate::ingame::{
    HardDropEvent,
    FixEvent,
};
use crate::ingame::utils::prelude::*;

/// ブロックを一番下に固定する関数
/// 衝突判定が出るまで、ブロックを下に移動させて固定する
pub fn block_harddrop(
    _was_harddrop: On<HardDropEvent>,
    mut commands: Commands,
    mut player_query: Query<&mut Transform, (With<PlayerBlock>, Without<Block>)>,
    block_query: Query<&Transform, With<Block>>,
) {
    info_once!("block_harddrop");

    // 衝突フラグ
    let mut collision = false;
    // 現在のステップ数（移動距離）
    let mut step = 0;

    // フィールドの下限を計算
    let field_boundary = FIELD_POSITION.y - FIELD_SIZE.y / 2.0;

    while !collision && step < BLOCK_MAP.len() {
        for player_transform in &mut player_query {
            let player_x = player_transform.translation.x;
            let player_y = player_transform.translation.y - GRID_SIZE * step as f32;

            for block_transform in &block_query {
                let block_x = block_transform.translation.x;
                let block_y = block_transform.translation.y;

                // プレイヤーブロックがブロックに衝突
                if player_x == block_x && player_y - GRID_SIZE == block_y {
                    collision = true;
                }
            }

            // プレイヤーブロックがフィールドの下限に衝突
            if player_y - GRID_SIZE < field_boundary {
                collision = true;
            }
        }

        // 衝突しなかったらステップを増加
        if !collision {
            step += 1;
        }
    }

    // 現在動かしているブロックを移動
    for mut transform in &mut player_query {
        transform.translation.y -= GRID_SIZE * step as f32;
    }

    // ブロックを固定
    commands.trigger(FixEvent);
}
