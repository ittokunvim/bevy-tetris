use bevy::prelude::*;

use crate::{
    BLOCK_SPEED,
    MoveEvent,
    RotationEvent,
    Direction,
    FallingTimer,
};

const KEY_BLOCK_LEFT_1: KeyCode = KeyCode::ArrowLeft;
const KEY_BLOCK_LEFT_2: KeyCode = KeyCode::KeyA;
const KEY_BLOCK_RIGHT_1: KeyCode = KeyCode::ArrowRight;
const KEY_BLOCK_RIGHT_2: KeyCode = KeyCode::KeyD;
const KEY_BLOCK_BOTTOM_1: KeyCode = KeyCode::ArrowDown;
const KEY_BLOCK_BOTTOM_2: KeyCode = KeyCode::KeyS;
const KEY_BLOCK_ROTATION_1: KeyCode = KeyCode::ArrowUp;
const KEY_BLOCK_ROTATION_2: KeyCode = KeyCode::KeyW;

fn move_event(
    mut events: EventWriter<MoveEvent>,
    mut timer: ResMut<FallingTimer>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    let mut closure = |direction: Direction| {
        events.send(MoveEvent(direction));
        if direction == Direction::Bottom {
            timer.0 = FallingTimer::update_timer(BLOCK_SPEED / 2.0);
        }
    };
    for key in keyboard_input.get_just_pressed() {
        match key {
            &KEY_BLOCK_LEFT_1   | &KEY_BLOCK_LEFT_2   => closure(Direction::Left),
            &KEY_BLOCK_RIGHT_1  | &KEY_BLOCK_RIGHT_2  => closure(Direction::Right),
            &KEY_BLOCK_BOTTOM_1 | &KEY_BLOCK_BOTTOM_2 => closure(Direction::Bottom),
            _ => {},
        }
    }
    for key in keyboard_input.get_just_released() {
        if key == &KEY_BLOCK_BOTTOM_1 || key == &KEY_BLOCK_BOTTOM_2 {
            timer.0 = FallingTimer::update_timer(BLOCK_SPEED);
        }
    }
}

fn rotation_event(
    mut events: EventWriter<RotationEvent>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    for key in keyboard_input.get_just_pressed() {
        if key == &KEY_BLOCK_ROTATION_1 || key == &KEY_BLOCK_ROTATION_2 {
            events.send(RotationEvent(Direction::Right));
        }
    }
}

pub struct KeyPlugin;

impl Plugin for KeyPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (
                move_event,
                rotation_event,
            ))
        ;
    }
}
