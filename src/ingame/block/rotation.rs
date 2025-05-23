use bevy::prelude::*;

use crate::GRID_SIZE;
use crate::ingame::{
    FIELD_SIZE,
    FIELD_POSITION,
    RotationEvent,
    Direction,
    FallingTimer,
};
use super::{
    MAX_BLOCK_COUNT,
    MAX_COLLISION_COUNT,
    CurrentBlock,
    PlayerBlock,
    Block,
};

/// ブロックの回転を管理する関数
/// `RotationEvent`を受け取り、ブロックの位置を更新し、
/// 必要に応じてブロックの衝突を処理します
pub fn block_rotation(
    mut events: EventReader<RotationEvent>,
    mut timer: ResMut<FallingTimer>,
    mut player_query: Query<(&PlayerBlock, &mut Transform), (With<PlayerBlock>, Without<Block>)>,
    mut current_block: ResMut<CurrentBlock>,
    block_query: Query<&Transform, With<Block>>,
) {
    info_once!("block_rotation");

    for event in events.read() {
        let direction = event.0;
        let mut count = 0;
        let mut collision_x = 0.0;
        let mut collision_y = 0.0;

        // タイマーをリセット
        timer.reset();

        // 現在のブロックIDを更新
        current_block.blockid = match direction {
            Direction::Right => (current_block.blockid + 1) % MAX_BLOCK_COUNT,
            Direction::Left  => (current_block.blockid + MAX_BLOCK_COUNT - 1) % MAX_BLOCK_COUNT,
            _ => current_block.blockid,
        };

        // 衝突をチェック
        for (player, mut _player_transform) in &mut player_query {
            while count < MAX_COLLISION_COUNT {
                // 回転時のブロックの位置を取得
                let position = current_block.position(player.0);

                // フィールド左側の衝突判定
                if position.x < FIELD_POSITION.x - FIELD_SIZE.x / 2.0 {
                    current_block.pos.x += GRID_SIZE;
                    collision_x += GRID_SIZE;
                    count += 1;
                }
                // フィールド右側の衝突判定
                else if position.x > FIELD_POSITION.x + FIELD_SIZE.x / 2.0 {
                    current_block.pos.x -= GRID_SIZE;
                    collision_x -= GRID_SIZE;
                    count += 1;
                }
                // フィールド下側の衝突判定
                else if position.y < FIELD_POSITION.y - FIELD_SIZE.y / 2.0 {
                    current_block.pos.y += GRID_SIZE;
                    collision_y += GRID_SIZE;
                    count += 1;
                }
                // ブロック同士の衝突判定
                else if block_query.iter().any(|block_transform|
                    position == block_transform.translation
                ) {
                    current_block.pos.y += GRID_SIZE;
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
            current_block.blockid = match direction {
                Direction::Right => (current_block.blockid + MAX_BLOCK_COUNT - 1) % MAX_BLOCK_COUNT,
                Direction::Left  => (current_block.blockid + 1) % MAX_BLOCK_COUNT,
                _ => current_block.blockid,
            };
            // 現在のブロック位置をリセット
            current_block.pos.x -= collision_x;
            current_block.pos.y -= collision_y;
            return;
        }
        // ブロックを回転させる
        for (player, mut player_transform) in &mut player_query {
            player_transform.translation = current_block.position(player.0);
        }
    }
}

