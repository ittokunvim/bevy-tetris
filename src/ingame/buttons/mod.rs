use bevy::prelude::*;

use crate::AppState;

mod spawn;

#[derive(Component, Debug)]
struct KeyButton;

#[derive(Component, Debug)]
struct MoveLeftButton;

#[derive(Component, Debug)]
struct MoveRightButton;

#[derive(Component, Debug)]
struct RotateLeftButton;

#[derive(Component, Debug)]
struct RotateRightButton;

#[derive(Component, Debug)]
struct HoldButton;

#[derive(Component, Debug)]
struct FallButton;

#[derive(Component, Debug)]
struct FixButton;

pub struct ButtonsPlugin;

impl Plugin for ButtonsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::InGame), spawn::setup)
        ;
    }
}
