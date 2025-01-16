use bevy::prelude::*;

use crate::AppState;
use crate::block::{
    MoveEvent as BlockMoveEvent,
    Direction as BlockDirection,
};

const KEY_BLOCK_LEFT_1: KeyCode = KeyCode::ArrowLeft;
const KEY_BLOCK_LEFT_2: KeyCode = KeyCode::KeyA;
const KEY_BLOCK_RIGHT_1: KeyCode = KeyCode::ArrowRight;
const KEY_BLOCK_RIGHT_2: KeyCode = KeyCode::KeyD;

pub fn block_movement(
    mut events: EventWriter<BlockMoveEvent>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    let mut closure = |direction: BlockDirection| {
        // debug!("block_movement: send event");
        events.send(BlockMoveEvent(direction));
    };

    for key in keyboard_input.get_just_pressed() {
        match key {
            &KEY_BLOCK_LEFT_1  | &KEY_BLOCK_LEFT_2  => closure(BlockDirection::Left),
            &KEY_BLOCK_RIGHT_1 | &KEY_BLOCK_RIGHT_2 => closure(BlockDirection::Right),
            _ => {},
        }
    }
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (
                block_movement,
                crate::block::movement::falling,
                crate::wall::check_for_wall,
                crate::block::collision::check_for_collision,
                crate::block::movement::movement,
            ).chain().run_if(in_state(AppState::Ingame)))
        ;
    }
}
