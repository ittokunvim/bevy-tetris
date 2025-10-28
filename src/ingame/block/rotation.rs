use bevy::prelude::*;

use crate::GRID_SIZE;
use crate::ingame::{
    BlockRotated,
    Direction,
    FallingTimer,
};
use crate::ingame::utils::prelude::*;

/// ブロックの回転を管理する関数
/// `RotationEvent`を受け取り、ブロックの位置を更新し、
/// 必要に応じてブロックの衝突を処理します
pub fn block_rotation(
    rotated: On<BlockRotated>,
    mut falling_timer: ResMut<FallingTimer>,
    mut player_query: Query<(&PlayerBlock, &mut Transform), (With<PlayerBlock>, Without<Block>)>,
    mut currentblock: ResMut<CurrentBlocks>,
    block_query: Query<&Transform, With<Block>>,
) {
    info_once!("block_rotation");

    let direction = rotated.0;

    // タイマーをリセット
    falling_timer.reset();

    // 現在のブロックIDを更新
    currentblock.blockid = match direction {
        Direction::Right => (currentblock.blockid + 1) % MAX_BLOCK_COUNT,
        Direction::Left  => (currentblock.blockid + MAX_BLOCK_COUNT - 1) % MAX_BLOCK_COUNT,
        _ => currentblock.blockid,
    };

    // 衝突の回数をカウント
    let mut count = 0;
    // X軸の動いた回数
    let mut step_x = 0;
    // Y軸の動いた回数
    let mut step_y = 0;
    // 衝突をチェック
    for (player, mut _player_transform) in &mut player_query {
        while count < MAX_COLLISION_COUNT {
            // 回転時のブロックの位置を取得
            let position = currentblock.position(player.0);

            // フィールド左側の衝突判定
            if position.x < FIELD_POSITION.x - FIELD_SIZE.x / 2.0 {
                currentblock.pos.x += GRID_SIZE;
                step_x += 1;
                count += 1;
            }
            // フィールド右側の衝突判定
            else if position.x > FIELD_POSITION.x + FIELD_SIZE.x / 2.0 {
                currentblock.pos.x -= GRID_SIZE;
                step_x -= 1;
                count += 1;
            }
            // フィールド下側の衝突判定
            else if position.y < FIELD_POSITION.y - FIELD_SIZE.y / 2.0 {
                currentblock.pos.y += GRID_SIZE;
                step_y += 1;
                count += 1;
            }
            // ブロック同士の衝突判定
            else if block_query.iter().any(|block_transform|
                position == block_transform.translation
            ) {
                currentblock.pos.y += GRID_SIZE;
                step_y += 1;
                count += 1;
            }
            // 衝突がなければループを抜ける
            else { break; }
        }
    }

    // もし衝突判定が規定回数以上あった場合、回転を行わない
    if count >= MAX_COLLISION_COUNT {
        // 現在のブロックIDをリセット
        currentblock.blockid = match direction {
            Direction::Right => (currentblock.blockid + MAX_BLOCK_COUNT - 1) % MAX_BLOCK_COUNT,
            Direction::Left  => (currentblock.blockid + 1) % MAX_BLOCK_COUNT,
            _ => currentblock.blockid,
        };
        // 現在のブロック位置をリセット
        currentblock.pos.x -= GRID_SIZE * step_x as f32;
        currentblock.pos.y -= GRID_SIZE * step_y as f32;
        return;
    }

    // ブロックを回転させる
    for (player, mut player_transform) in &mut player_query {
        player_transform.translation = currentblock.position(player.0);
    }
}
