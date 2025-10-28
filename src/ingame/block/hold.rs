use bevy::prelude::*;

use crate::ingame::{
    BlockSpawned,
    BlockHolded,
};
use crate::ingame::utils::prelude::*;

/// ブロックをホールドした時の挙動を決める関数
/// ホールドされたらプレイヤーブロックを削除して
/// 新しいプレイヤーブロックを再度生成する
pub fn block_hold(
    holded: On<BlockHolded>,
    mut commands: Commands,
    mut holdblocks: ResMut<HoldBlocks>,
    player_query: Query<Entity, With<PlayerBlock>>,
) {
    info_once!("block_hold");

    let blocktype = holded.0;

    // プレイヤーブロックを削除する
    for entity in &player_query {
        commands.entity(entity).despawn();
    }

    if let Some(blocktype) = holdblocks.blocktype {
        commands.trigger(BlockSpawned(Some(blocktype)));
    } else {
        commands.trigger(BlockSpawned(None));
    }

    holdblocks.blocktype = Some(blocktype);
}
