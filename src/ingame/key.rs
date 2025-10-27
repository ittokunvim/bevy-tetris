use bevy::prelude::*;

use crate::AppState;
use super::{
    MoveEvent,
    RotationEvent,
    HardDropEvent,
    HoldEvent,
    Direction,
};
use super::utils::prelude::*;

const KEY_BLOCK_MOVE_LEFT: KeyCode = KeyCode::ArrowLeft;
const KEY_BLOCK_MOVE_RIGHT: KeyCode = KeyCode::ArrowRight;
const KEY_BLOCK_MOVE_BOTTOM: KeyCode = KeyCode::ArrowDown;
const KEY_BLOCK_ROTATION_LEFT: KeyCode = KeyCode::KeyZ;
const KEY_BLOCK_ROTATION_RIGHT: KeyCode = KeyCode::ArrowUp;
const KEY_BLOCK_HARDDROP: KeyCode = KeyCode::Space;
const KEY_BLOCK_HOLD: KeyCode = KeyCode::KeyC;

/// ブロック左移動キーが入力された時の挙動を決める関数
fn key_block_moveleft(
    mut commands: Commands,
    mut moveleft_timer: ResMut<MoveLeftTimer>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    info_once!("key_block_moveleft");

    // ブロック左移動キー入力時
    if keyboard_input.just_pressed(KEY_BLOCK_MOVE_LEFT) {
        // ブロック左移動イベントを発火
        commands.trigger(MoveEvent(Direction::Left));
    }

    // ブロック左移動キー長押し時
    if keyboard_input.pressed(KEY_BLOCK_MOVE_LEFT) {
        // ブロック左移動タイマーを進める
        moveleft_timer.0.tick(time.delta());
        // ブロック左移動タイマーが切れたら、タイマーをリセットし、イベントを発火
        if moveleft_timer.0.elapsed_secs() > BLOCK_MOVE_SPEED {
            moveleft_timer.0.reset();
            commands.trigger(MoveEvent(Direction::Left));
        }
    }

    // ブロック左移動キーを離した時
    if keyboard_input.just_released(KEY_BLOCK_MOVE_LEFT) {
        // ブロック左移動タイマーをリセット
        moveleft_timer.0.reset();
    }
}

/// ブロック右移動キーが入力された時の挙動を決める関数
fn key_block_moveright(
    mut commands: Commands,
    mut moveright_timer: ResMut<MoveRightTimer>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    info_once!("key_block_moveright");

    // ブロック右移動キー入力時
    if keyboard_input.just_pressed(KEY_BLOCK_MOVE_RIGHT) {
        // ブロック右移動イベントを発火
        commands.trigger(MoveEvent(Direction::Right));
    }

    // ブロック右移動キー長押し時
    if keyboard_input.pressed(KEY_BLOCK_MOVE_RIGHT) {
        // ブロック右移動タイマーを進める
        moveright_timer.0.tick(time.delta());
        // ブロック右移動タイマーが切れたら、タイマーをリセットし、イベントを発火
        if moveright_timer.0.elapsed_secs() > BLOCK_MOVE_SPEED {
            moveright_timer.0.reset();
            commands.trigger(MoveEvent(Direction::Right));
        }
    }

    // ブロック右移動キーを離した時
    if keyboard_input.just_released(KEY_BLOCK_MOVE_RIGHT) {
        // ブロック右移動タイマーをリセット
        moveright_timer.0.reset();
    }
}

/// ブロック下移動キーが入力された時の挙動を決める関数
fn key_block_movebottom(
    mut commands: Commands,
    mut falling_timer: ResMut<FallingTimer>,
    mut movebottom_timer: ResMut<MoveBottomTimer>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    info_once!("key_block_movebottom");

    // ブロック下移動キー入力時
    if keyboard_input.just_pressed(KEY_BLOCK_MOVE_BOTTOM) {
        // ブロック下移動イベントを発火
        commands.trigger(MoveEvent(Direction::Bottom));
        // ブロック落下タイマーを一時停止し、タイマーをリセット
        falling_timer.0.pause();
        falling_timer.0.reset();
    }

    // ブロック下移動キー長押し時
    if keyboard_input.pressed(KEY_BLOCK_MOVE_BOTTOM) {
        // ブロック下移動タイマーを進める
        movebottom_timer.0.tick(time.delta());
        // ブロック下移動タイマーが切れたら、タイマーをリセットし、イベントを発火
        if movebottom_timer.0.elapsed_secs() > BLOCK_MOVE_SPEED {
            movebottom_timer.0.reset();
            commands.trigger(MoveEvent(Direction::Bottom));
        }
    }

    // ブロック下移動キー離した時
    if keyboard_input.just_released(KEY_BLOCK_MOVE_BOTTOM) {
        // ブロック下移動タイマーをリセットし、一時停止を解除
        movebottom_timer.0.reset();
        falling_timer.0.unpause();
    }
}

/// ブロック左回転キーが入力された時の挙動を決める関数
fn key_block_rotationleft(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    info_once!("key_block_rotationleft");

    // ブロック左回転キーが押されたら、イベントを発火
    if keyboard_input.just_pressed(KEY_BLOCK_ROTATION_LEFT) {
        commands.trigger(RotationEvent(Direction::Left));
    }
}

/// ブロック右回転キーが入力された時の挙動を決める関数
fn key_block_rotationright(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    info_once!("key_block_rotationright");

    // ブロック右回転キーが押されたら、イベントを発火
    if keyboard_input.just_pressed(KEY_BLOCK_ROTATION_RIGHT) {
        commands.trigger(RotationEvent(Direction::Right));
    }
}

/// ハードドロップキーが入力された時の挙動を決める関数
fn key_block_harddrop(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    info_once!("key_block_harddrop");

    // ハードドロップキーが押されたら、イベントを発火
    if keyboard_input.just_pressed(KEY_BLOCK_HARDDROP) {
        commands.trigger(HardDropEvent);
    }
}

/// ブロックホールドキーが入力された時の挙動を決める関数
fn key_block_hold(
    mut commands: Commands,
    mut holdblocks: ResMut<HoldBlocks>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    currentblock: Res<CurrentBlocks>,
) {
    info_once!("key_block_hold");

    // ブロックホールドキーが押されたら
   if keyboard_input.just_pressed(KEY_BLOCK_HOLD) {
        // ホールドが許可されていたら、許可を取り消し、イベントを発火
        if holdblocks.can_hold {
            holdblocks.can_hold = false;
            commands.trigger(HoldEvent(currentblock.blocktype));
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
                key_block_rotationleft,
                key_block_rotationright,
                key_block_harddrop,
                key_block_hold,
            ).run_if(in_state(AppState::InGame)))
        ;
    }
}
