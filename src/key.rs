use bevy::prelude::*;

use crate::{
    BLOCK_SPEED,
    MoveEvent,
    RotationEvent,
    HardDropEvent,
    Direction,
    FallingTimer,
    AppState,
};

const KEY_BLOCK_LEFT_1: KeyCode = KeyCode::ArrowLeft;
const KEY_BLOCK_LEFT_2: KeyCode = KeyCode::KeyA;
const KEY_BLOCK_RIGHT_1: KeyCode = KeyCode::ArrowRight;
const KEY_BLOCK_RIGHT_2: KeyCode = KeyCode::KeyD;
const KEY_BLOCK_BOTTOM_1: KeyCode = KeyCode::ArrowDown;
const KEY_BLOCK_BOTTOM_2: KeyCode = KeyCode::KeyS;
const KEY_BLOCK_ROTATION_LEFT: KeyCode = KeyCode::KeyZ;
const KEY_BLOCK_ROTATION_RIGHT: KeyCode = KeyCode::ArrowUp;
const KEY_BLOCK_HARDDROP: KeyCode = KeyCode::Space;

fn move_event(
    mut events: EventWriter<MoveEvent>,
    mut timer: ResMut<FallingTimer>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    info_once!("move_event");

    let mut closure = |direction: Direction| {
        events.send(MoveEvent(direction));
        if direction == Direction::Bottom {
            timer.0 = FallingTimer::update_timer(BLOCK_SPEED / 2.0);
        }
    };
    for key in keyboard_input.get_just_pressed() {
        match *key {
            KEY_BLOCK_LEFT_1   | KEY_BLOCK_LEFT_2   => closure(Direction::Left),
            KEY_BLOCK_RIGHT_1  | KEY_BLOCK_RIGHT_2  => closure(Direction::Right),
            KEY_BLOCK_BOTTOM_1 | KEY_BLOCK_BOTTOM_2 => closure(Direction::Bottom),
            _ => {},
        }
    }
    for key in keyboard_input.get_just_released() {
        if *key == KEY_BLOCK_BOTTOM_1 || *key == KEY_BLOCK_BOTTOM_2 {
            timer.0 = FallingTimer::update_timer(BLOCK_SPEED);
        }
    }
}

fn rotation_event(
    mut events: EventWriter<RotationEvent>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    info_once!("rotation_event");

    let mut closure = |direction: Direction| {
        events.send(RotationEvent(direction));
    };
    for key in keyboard_input.get_just_pressed() {
        match *key {
            KEY_BLOCK_ROTATION_LEFT  => closure(Direction::Left),
            KEY_BLOCK_ROTATION_RIGHT => closure(Direction::Right),
            _ => {},
        };
    }
}

fn harddrop_event(
    mut events: EventWriter<HardDropEvent>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    info_once!("harddrop_event");

    for key in  keyboard_input.get_just_pressed() {
        if *key == KEY_BLOCK_HARDDROP {
            events.send_default();
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
                harddrop_event,
            ).run_if(in_state(AppState::InGame)))
        ;
    }
}
