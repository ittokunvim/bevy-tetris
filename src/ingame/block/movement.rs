use bevy::prelude::*;

use crate::GRID_SIZE;
use crate::ingame::{
    MoveEvent,
    FixEvent,
    Direction,
    FallingTimer,
};
use crate::ingame::utils::prelude::*;

/// ブロックの落下を管理する関数
/// `FallingTimer`を使用して一定間隔でブロックを下に移動させる
pub fn block_falling(
    mut commands: Commands,
    mut timer: ResMut<FallingTimer>,
    time: Res<Time>,
) {
    info_once!("block_falling");

    // タイマーを進める
    timer.tick(time.delta());

    // タイマーが終わったかチェック
    if !timer.just_finished() {
        return;
    }

    // ブロックを下に移動させるイベントを送信
    commands.trigger(MoveEvent(Direction::Bottom));
}

/// ブロックの移動を管理する関数
/// `MoveEvent`を受け取り、ブロックの位置を更新し、
/// 必要に応じてブロックを固定する
pub fn block_movement(
    moved: On<MoveEvent>,
    mut commands: Commands,
    mut player_query: Query<&mut Transform, (With<PlayerBlock>, Without<Block>)>,
    mut currentblock: ResMut<CurrentBlocks>,
    block_query: Query<&Transform, With<Block>>,
) {
    info_once!("block_movement");

    let direction = moved.0;

    // フィールドの衝突をチェック
    for player_transform in &mut player_query {
        let player_x = player_transform.translation.x;
        let player_y = player_transform.translation.y;

        match direction {
            Direction::Left => {
                if player_x - GRID_SIZE < FIELD_POSITION.x - FIELD_SIZE.x / 2.0 {
                    return;
                }
            }
            Direction::Right => {
                if player_x + GRID_SIZE > FIELD_POSITION.x + FIELD_SIZE.x / 2.0 {
                    return;
                }
            }
            Direction::Bottom => {
                if player_y - GRID_SIZE < FIELD_POSITION.y - FIELD_SIZE.y / 2.0 {
                    // ブロックがそこに達した場合、ブロックを固定
                    commands.trigger(FixEvent);
                    return;
                }
            }
        }

        // ブロックの衝突をチェック
        for block_transform in &block_query {
            let block_x = block_transform.translation.x;
            let block_y = block_transform.translation.y;

            match direction {
                Direction::Left => {
                    if player_x - GRID_SIZE == block_x && player_y == block_y {
                        return;
                    }
                }
                Direction::Right => {
                    if player_x + GRID_SIZE == block_x && player_y == block_y {
                        return;
                    }
                }
                Direction::Bottom => {
                    if player_x == block_x && player_y - GRID_SIZE == block_y {
                        // ブロックが底に達した場合、ブロックを固定
                        commands.trigger(FixEvent);
                        return;
                    }
                }
            }
        }
    }

    // 現在のブロック位置を更新
    match direction {
        Direction::Left   => currentblock.pos.x -= GRID_SIZE,
        Direction::Right  => currentblock.pos.x += GRID_SIZE,
        Direction::Bottom => currentblock.pos.y -= GRID_SIZE,
    }
    // ブロックを移動
    for mut transform in &mut player_query {
        match direction {
            Direction::Left   => transform.translation.x -= GRID_SIZE,
            Direction::Right  => transform.translation.x += GRID_SIZE,
            Direction::Bottom => transform.translation.y -= GRID_SIZE,
        }
    }
}
