use bevy::prelude::*;

use crate::ingame::{
    SpawnEvent,
    HoldEvent,
};
use crate::ingame::utils::prelude::*;

/// ブロックをホールドした時の挙動を決める関数
/// ホールドされたらプレイヤーブロックを削除して
/// 新しいプレイヤーブロックを再度生成する
pub fn block_hold(
    mut hold_events: EventReader<HoldEvent>,
    mut spawn_events: EventWriter<SpawnEvent>,
    mut commands: Commands,
    mut holdblocks: ResMut<HoldBlocks>,
    player_query: Query<Entity, With<PlayerBlock>>,
) {
    info_once!("block_hold");

    for event in hold_events.read() {
        let blocktype = event.0;

        // プレイヤーブロックを削除する
        for entity in &player_query {
            commands.entity(entity).despawn();
        }

        if let Some(blocktype) = holdblocks.blocktype {
            spawn_events.write(SpawnEvent(Some(blocktype)));
        } else {
            spawn_events.write_default();
        }

        holdblocks.blocktype = Some(blocktype);
   }
}
