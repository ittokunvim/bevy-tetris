use bevy::prelude::*;

use crate::{
    GRID_SIZE,
    AppState,
};
use super::{
    SpawnEvent,
    FixEvent,
};
use super::utils::prelude::*;

mod clear;
mod gizmos;
mod harddrop;
mod hold;
mod movement;
mod rotation;
mod spawn;

fn setup(mut events: EventWriter<SpawnEvent>) {
    info_once!("setup");

    events.send_default();
}

/// ゲームオーバーを管理する関数
/// `FixEvent`を受け取り、固定されたブロックから
/// ゲームオーバーになるかどうかチェックします
///
fn gameover(
    mut events: EventReader<FixEvent>,
    mut next_state: ResMut<NextState<AppState>>,
    query: Query<&Transform, With<PlayerBlock>>,
) {
    info_once!("gameover");

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

/// ブロックを全て削除する関数
/// ゲームオーバを抜けた時に実行される
fn despawn(
    mut commands: Commands,
    query: Query<Entity, With<Block>>,
) {
    info_once!("despawn");

    for entity in &query {
        commands.entity(entity).despawn();
    }
}

/// リソースをリセットする関数
/// ゲームオーバを抜けた時に実行される
fn reset(
    mut rotation_block: ResMut<CurrentBlock>,
    mut block_map: ResMut<BlockMap>,
) {
    info_once!("reset");

    *rotation_block = CurrentBlock::new();
    *block_map = BlockMap(BLOCK_MAP);
}

pub struct BlockPlugin;

impl Plugin for BlockPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::InGame), setup)
            .add_systems(Update, (
                spawn::block_spawn,
                movement::block_falling,
                rotation::block_rotation,
                movement::block_movement,
                harddrop::block_harddrop,
                hold::block_hold,
                gizmos::draw_gizmos_block,
                gameover,
                clear::block_clear,
            ).chain().run_if(in_state(AppState::InGame)))
            .add_systems(OnExit(AppState::Gameover), despawn)
            .add_systems(OnExit(AppState::Gameover), reset)
        ;
    }
}
