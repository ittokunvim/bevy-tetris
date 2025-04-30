use bevy::prelude::*;

use crate::{
    BLOCK_MOVE_SPEED,
    MoveEvent,
    RotationEvent,
    HardDropEvent,
    Direction,
    FallingTimer,
    MoveLeftTimer,
    MoveBottomTimer,
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
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    info_once!("move_event");

    let mut closure = |direction: Direction| {
        events.send(MoveEvent(direction));
    };
    for key in keyboard_input.get_just_pressed() {
        match *key {
            KEY_BLOCK_RIGHT_1  | KEY_BLOCK_RIGHT_2  => closure(Direction::Right),
            _ => {},
        }
    }
}

fn key_block_moveleft(
    mut events: EventWriter<MoveEvent>,
    mut moveleft_timer: ResMut<MoveLeftTimer>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    info_once!("key_block_moveleft");

    let block_left_keys = [KEY_BLOCK_LEFT_1, KEY_BLOCK_LEFT_2];

    // ブロック左移動キー入力時
    if keyboard_input.any_just_pressed(block_left_keys) {
        // ブロック移動イベントを発火
        events.send(MoveEvent(Direction::Left));
    }

    // ブロック左移動キー長押し時
    if keyboard_input.any_pressed(block_left_keys) {
        // ブロック左移動タイマーを進める
        moveleft_timer.0.tick(time.delta());
        // タイマーが切れたら
        if moveleft_timer.0.elapsed_secs() > BLOCK_MOVE_SPEED {
            // ブロック左移動タイマーをリセット
            moveleft_timer.0.reset();
            // ブロック移動イベントを発火
            events.send(MoveEvent(Direction::Left));
        }
    }

    // ブロック左移動キーを離した時
    if keyboard_input.any_just_released(block_left_keys) {
        // ブロック下移動タイマーをリセット
        moveleft_timer.0.reset();
    }
}

fn key_block_movebottom(
    mut events: EventWriter<MoveEvent>,
    mut falling_timer: ResMut<FallingTimer>,
    mut movebottom_timer: ResMut<MoveBottomTimer>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    info_once!("key_block_movebottom");

    let block_bottom_keys = [KEY_BLOCK_BOTTOM_1, KEY_BLOCK_BOTTOM_2];

    // ブロック下移動キー入力時
    if keyboard_input.any_just_pressed(block_bottom_keys) {
        // ブロック移動イベントを発火
        events.send(MoveEvent(Direction::Bottom));
        // ブロック落下タイマーを一時停止
        falling_timer.0.pause();
        // ブロック落下タイマーをリセット
        falling_timer.0.reset();
    }

    // ブロック下移動キー長押し時
    if keyboard_input.any_pressed(block_bottom_keys) {
        // ブロック下移動タイマーを進める
        movebottom_timer.0.tick(time.delta());

        if movebottom_timer.0.elapsed_secs() > BLOCK_MOVE_SPEED {
            // ブロック下移動タイマーをリセット
            movebottom_timer.0.reset();
            // ブロック移動イベントを発火
            events.send(MoveEvent(Direction::Bottom));
        }
    }

    // ブロック下移動キー離した時
    if keyboard_input.any_just_released(block_bottom_keys) {
        // ブロック下移動タイマーをリセット
        movebottom_timer.0.reset();
        // ブロック落下タイマーの一時停止を解除
        falling_timer.0.unpause();
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
                key_block_moveleft,
                key_block_movebottom,
                rotation_event,
                harddrop_event,
            ).run_if(in_state(AppState::InGame)))
        ;
    }
}
