use bevy::prelude::*;

use crate::{
    BLOCK_MOVE_SPEED,
    MoveEvent,
    RotationEvent,
    HardDropEvent,
    HoldEvent,
    Direction,
    FallingTimer,
    MoveLeftTimer,
    MoveRightTimer,
    MoveBottomTimer,
    AppState,
};
use crate::block::{
    CurrentBlock,
    HoldBlocks,
};

const KEY_BLOCK_MOVE_LEFT: KeyCode = KeyCode::ArrowLeft;
const KEY_BLOCK_MOVE_RIGHT: KeyCode = KeyCode::ArrowRight;
const KEY_BLOCK_MOVE_BOTTOM: KeyCode = KeyCode::ArrowDown;
const KEY_BLOCK_ROTATION_LEFT: KeyCode = KeyCode::KeyZ;
const KEY_BLOCK_ROTATION_RIGHT: KeyCode = KeyCode::ArrowUp;
const KEY_BLOCK_HARDDROP: KeyCode = KeyCode::Space;
const KEY_BLOCK_HOLD: KeyCode = KeyCode::KeyC;

fn key_block_moveleft(
    mut events: EventWriter<MoveEvent>,
    mut moveleft_timer: ResMut<MoveLeftTimer>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    info_once!("key_block_moveleft");

    // ブロック左移動キー入力時
    if keyboard_input.just_pressed(KEY_BLOCK_MOVE_LEFT) {
        // ブロック移動イベントを発火
        events.send(MoveEvent(Direction::Left));
    }

    // ブロック左移動キー長押し時
    if keyboard_input.pressed(KEY_BLOCK_MOVE_LEFT) {
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
    if keyboard_input.just_released(KEY_BLOCK_MOVE_LEFT) {
        // ブロック左移動タイマーをリセット
        moveleft_timer.0.reset();
    }
}

fn key_block_moveright(
    mut events: EventWriter<MoveEvent>,
    mut moveright_timer: ResMut<MoveRightTimer>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    info_once!("key_block_moveright");

    // ブロック右移動キー入力時
    if keyboard_input.just_pressed(KEY_BLOCK_MOVE_RIGHT) {
        // ブロック移動イベントを発火
        events.send(MoveEvent(Direction::Right));
    }

    // ブロック右移動キー長押し時
    if keyboard_input.pressed(KEY_BLOCK_MOVE_RIGHT) {
        // ブロック右移動タイマーを進める
        moveright_timer.0.tick(time.delta());
        // タイマーが切れたら
        if moveright_timer.0.elapsed_secs() > BLOCK_MOVE_SPEED {
            // ブロック右移動タイマーをリセット
            moveright_timer.0.reset();
            // ブロック移動イベントを発火
            events.send(MoveEvent(Direction::Right));
        }
    }

    // ブロック右移動キーを離した時
    if keyboard_input.just_released(KEY_BLOCK_MOVE_RIGHT) {
        // ブロック右移動タイマーをリセット
        moveright_timer.0.reset();
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

    // ブロック下移動キー入力時
    if keyboard_input.just_pressed(KEY_BLOCK_MOVE_BOTTOM) {
        // ブロック移動イベントを発火
        events.send(MoveEvent(Direction::Bottom));
        // ブロック落下タイマーを一時停止
        falling_timer.0.pause();
        // ブロック落下タイマーをリセット
        falling_timer.0.reset();
    }

    // ブロック下移動キー長押し時
    if keyboard_input.pressed(KEY_BLOCK_MOVE_BOTTOM) {
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
    if keyboard_input.just_released(KEY_BLOCK_MOVE_BOTTOM) {
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

fn key_block_hold(
    mut events: EventWriter<HoldEvent>,
    mut holdblocks: ResMut<HoldBlocks>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    currentblock: Res<CurrentBlock>,
) {
    info_once!("key_block_hold");

   if keyboard_input.just_pressed(KEY_BLOCK_HOLD) {
        // ホールドが許可されている場合のみ実行
        if holdblocks.can_hold {
            holdblocks.can_hold = false;
            events.send(HoldEvent(currentblock.blocktype));
        }
    }
}

pub struct KeyPlugin;

impl Plugin for KeyPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (
                key_block_moveright,
                key_block_moveleft,
                key_block_movebottom,
                rotation_event,
                harddrop_event,
                key_block_hold,
            ).run_if(in_state(AppState::InGame)))
        ;
    }
}
