use bevy::prelude::*;

use super::RotationEvent;

pub fn rotation(
    mut events: EventReader<RotationEvent>,
) {
    if events.is_empty() { return }
    events.clear();
    // debug!("read RotationEvent");
}

// pub struct RotationPlugin;

// impl Plugin for RotationPlugin {
//     fn build(&self, app: &mut App) {
//         app
//         ;
//     }
// }
