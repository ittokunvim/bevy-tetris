use bevy::prelude::*;

use crate::utils::blockdata::*;
use super::{
    RotationEvent,
    CurrentBlock,
};

pub fn rotation(
    mut events: EventReader<RotationEvent>,
    mut current_block: ResMut<CurrentBlock>,
) {
    if events.is_empty() { return }
    events.clear();
    // debug!("read RotationEvent");
    if current_block.id >= MAX_BLOCKDATA - 1 { current_block.id = 0 }
    else { current_block.id += 1 }
    // trace!("current_block.id: {}", current_block.id);
    assert!(current_block.id < MAX_BLOCKDATA);
}

// pub struct RotationPlugin;

// impl Plugin for RotationPlugin {
//     fn build(&self, app: &mut App) {
//         app
//         ;
//     }
// }
