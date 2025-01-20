use bevy::prelude::*;

use super::BlockRotationEvent;

const KEY_BLOCK_ROTATION: KeyCode = KeyCode::Space;

pub fn rotation(
    mut events: EventWriter<BlockRotationEvent>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboard_input.just_pressed(KEY_BLOCK_ROTATION) {
        // debug!("send BlockRotationEvent");
        events.send_default();
    }
}
