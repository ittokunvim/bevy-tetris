use bevy::prelude::*;

use crate::{
    GRID_SIZE,
    FIELD_SIZE,
    FIELD_POSITION,
    RotationEvent,
    Direction,
    FallingTimer,
};
use crate::block::{
    MAX_BLOCK_COUNT,
    MAX_COLLISION_COUNT,
    RotationBlock,
    PlayerBlock,
    Block,
};

/// ブロックの回転を管理する関数
/// `RotationEvent`を受け取り、ブロックの位置を更新し、
/// 必要に応じてブロックの衝突を処理します
///
pub fn block_rotation(
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

