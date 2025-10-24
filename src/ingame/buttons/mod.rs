use bevy::prelude::*;

use crate::AppState;

mod spawn;
mod update;

#[derive(Component, Debug)]
struct KeyButton;

#[derive(Component, Debug)]
struct MoveLeftButton;

#[derive(Component, Debug)]
struct MoveRightButton;

#[derive(Component, Debug)]
struct MoveBottomButton;

#[derive(Component, Debug)]
struct RotateLeftButton;

#[derive(Component, Debug)]
struct RotateRightButton;

#[derive(Component, Debug)]
struct HoldButton;

#[derive(Component, Debug)]
struct HarddropButton;

pub struct ButtonsPlugin;

impl Plugin for ButtonsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::InGame), spawn::setup)
            .add_systems(Update, (
                update::button_block_moveleft,
                update::button_block_moveright,
                update::button_block_movebottom,
                update::button_block_rotationleft,
                update::button_block_rotationright,
                update::button_block_harddrop,
                update::button_block_hold,
            ).run_if(in_state(AppState::InGame)))
            .add_systems(OnExit(AppState::Gameover), update::despawn)
        ;
    }
}
