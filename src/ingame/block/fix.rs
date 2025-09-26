use bevy::prelude::*;

use crate::{
    GRID_SIZE,
    AppState,
    Score,
};
use crate::ingame::{
    SpawnEvent,
    FixEvent,
};
use crate::ingame::utils::prelude::*;

/// ブロックの削除を管理する関数
/// `FixEvent`を受け取り、プレイヤーブロックを固定ブロックに変換し、
/// ブロックマップを更新して、ラインが揃った場合にブロックを削除します。
pub fn clear_block(
    mut fix_events: EventReader<FixEvent>,
    mut commands: Commands,
    mut player_query: Query<(Entity, &mut Transform), (With<PlayerBlock>, Without<Block>)>,
    mut block_query: Query<(Entity, &mut Transform), (With<Block>, Without<PlayerBlock>)>,
    mut blockmap: ResMut<BlockMap>,
    mut score: ResMut<Score>,
    mut spawn_events: EventWriter<SpawnEvent>,
) {
    info_once!("clear_block");

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
        blockmap.insert(player_transform.translation.truncate());
    }

    let map = blockmap.0;
    for (index, row) in map.iter().enumerate() {
        // ブロックマップで横1列に1が並んでいたら、その列のブロックを削除する
        if *row == [1; 10] {
            // 削除後のブロックマップを更新
            blockmap.clearline(index);

            // 削除するブロックのy座標を定義
            let y = FIELD_LEFT_TOP.y + GRID_SIZE * 4.0 - GRID_SIZE * index as f32;

            // プレイヤーブロックをチェックし、削除するY座標と同じなら削除
            for (player_entity, mut player_transform) in &mut player_query {
                if player_transform.translation.y == y {
                    commands.entity(player_entity).despawn();
                }
                if player_transform.translation.y > y {
                    player_transform.translation.y -= GRID_SIZE;
                }
            }

            // 固定ブロックをチェックし、削除するY座標と同じなら削除
            for (block_entity, mut block_transform) in &mut block_query {
                if block_transform.translation.y == y {
                    commands.entity(block_entity).despawn();
                }
                if block_transform.translation.y > y {
                    block_transform.translation.y -= GRID_SIZE;
                }
            }

            // スコアを更新
            **score += 1;
        }
    }

    // ブロックを生成するイベントを送信
    spawn_events.write_default();
}

/// ホールドができるかどうか管理する関数
pub fn enable_hold(
    mut events: EventReader<FixEvent>,
    mut holdblocks: ResMut<HoldBlocks>,
) {
    info_once!("enable_hold");

    // イベントをチェック
    if events.is_empty() {
        return;
    }

    // イベントをクリア
    events.clear();

    // ホールドを有効にする
    holdblocks.can_hold = true;
}

/// ゲームオーバーを管理する関数
/// 固定されたブロックからゲームオーバーになるかどうかチェックします
pub fn check_gameover(
    mut events: EventReader<FixEvent>,
    mut next_state: ResMut<NextState<AppState>>,
    query: Query<&Transform, With<Block>>,
) {
    info_once!("check_gameover");

    // イベントをチェック
    if events.is_empty() {
        return;
    }

    // イベントをクリア
    events.clear();

    // ゲームオーバーかどうか判定する
    for transform in &query {
        let pos = transform.translation;
        if pos.y >= FIELD_LEFT_TOP.y {
            if pos.x == FIELD_LEFT_TOP.x + GRID_SIZE * 5.0
            || pos.x == FIELD_LEFT_TOP.x + GRID_SIZE * 6.0 {
                next_state.set(AppState::Gameover);
                return;
            }
        }
    }
}
