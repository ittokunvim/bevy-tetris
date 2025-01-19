use bevy::prelude::*;

use crate::AppState;
use crate::block::RotationEvent as BlockRotationEvent;

const KEY_BLOCK_ROTATION: KeyCode = KeyCode::Space;

fn block_rotation(
    mut events: EventWriter<BlockRotationEvent>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboard_input.just_pressed(KEY_BLOCK_ROTATION) {
        // debug!("send BlockRotationEvent");
        events.send_default();
    }
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (
                crate::wall::check_for_wall,
                crate::block::collision::check_for_collision,
            ).chain().run_if(in_state(AppState::Ingame)))
            .add_systems(Update, (
                block_rotation,
                crate::block::rotation::rotation,
            ).chain().run_if(in_state(AppState::Ingame)))
        ;
    }
}
