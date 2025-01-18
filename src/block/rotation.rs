use bevy::prelude::*;

use crate::utils::blockdata::*;
use super::{
    RotationEvent,
    CurrentBlock,
    PlayerBlock,
};

pub fn rotation(
    mut events: EventReader<RotationEvent>,
    mut current_block: ResMut<CurrentBlock>,
    mut query: Query<(&PlayerBlock, &mut Transform), With<PlayerBlock>>,
) {
    if events.is_empty() { return }
    events.clear();
    // debug!("read RotationEvent");
    if current_block.id >= MAX_BLOCKDATA - 1 { current_block.id = 0 }
    else { current_block.id += 1 }
    // trace!("current_block.id: {}", current_block.id);
    assert!(current_block.id < MAX_BLOCKDATA);

    let block = current_block.block;
    let blockdata_id = current_block.id;
    let init_pos = current_block.init_pos;

    for (player, mut transform) in &mut query {
        let block_id = player.0;
        transform.translation = block.position(init_pos, blockdata_id, block_id).extend(1.0);
    }
}

// pub struct RotationPlugin;

// impl Plugin for RotationPlugin {
//     fn build(&self, app: &mut App) {
//         app
//         ;
//     }
// }
