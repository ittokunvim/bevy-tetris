use bevy::prelude::*;

use crate::{
    AppState,
    WINDOW_SIZE,
    PATH_IMAGE_ANGLE_DOWN,
    PATH_IMAGE_ANGLE_LEFT,
    PATH_IMAGE_ANGLE_RIGHT,
    PATH_IMAGE_ANGLE_UP,
    PATH_IMAGE_FALL,
    PATH_IMAGE_HOLD,
};

use super::{
    Direction,
    MoveEvent,
};

const BUTTON_FIELD_SIZE: Vec2 = Vec2::new(WINDOW_SIZE.x, WINDOW_SIZE.y / 4.0);
const BUTTON_SIZE: f32 = 45.0;
const BUTTON_COLOR: Color = Color::srgb(1.0, 1.0, 1.0);
const BUTTON_MARGIN: f32 = 5.0;

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

impl KeyButton {
    /// ゲーム画面に配置するボタンの背景を生成します
    ///
    /// Returns:
    /// * `Self`: KeyButtonのインスタンス。
    /// * `Node`: 幅と高さを指定したノード。
    /// * `BackgroundColor`: 背景色
    fn from_board() -> (Self, Node, BackgroundColor) {
        (
            Self,
            Node {
                width: Val::Px(BUTTON_FIELD_SIZE.x),
                height: Val::Px(BUTTON_FIELD_SIZE.y),
                position_type: PositionType::Absolute,
                top: Val::Px(WINDOW_SIZE.y - BUTTON_FIELD_SIZE.y),
                ..Default::default()
            },
            BackgroundColor(Color::BLACK),
        )
    }

    /// 左側に配置するボタンを生成します
    fn from_left_button(x: f32, y: f32) -> (
        Self,
        Button,
        Node,
        BorderColor,
        BorderRadius,
        BackgroundColor,
    ) {
        (
            Self,
            Button,
            Node {
                width: Val::Px(BUTTON_SIZE),
                height: Val::Px(BUTTON_SIZE),
                border: UiRect::all(Val::Px(6.0)),
                position_type: PositionType::Absolute,
                left: Val::Px(x),
                top: Val::Px(y),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            BorderColor(Color::WHITE),
            BorderRadius::MAX,
            BackgroundColor(Color::WHITE),
        )
    }

    /// 右側に配置するボタンを生成します
    fn from_right_button(x: f32, y: f32) -> (
        Self,
        Button,
        Node,
        BorderColor,
        BorderRadius,
        BackgroundColor,
    ) {
        (
            Self,
            Button,
            Node {
                width: Val::Px(BUTTON_SIZE),
                height: Val::Px(BUTTON_SIZE),
                border: UiRect::all(Val::Px(6.0)),
                position_type: PositionType::Absolute,
                right: Val::Px(x),
                top: Val::Px(y),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            BorderColor(Color::WHITE),
            BorderRadius::MAX,
            BackgroundColor(Color::WHITE),
        )
    }

    /// ボタンの中に配置するアイコンを生成します
    fn from_icon(image: Handle<Image>) -> (Self, ImageNode, Node) {
        (
            Self,
            ImageNode::new(image),
            Node {
                width: Val::Px(BUTTON_SIZE),
                height: Val::Px(BUTTON_SIZE),
                ..Default::default()
            },
        )
    }
}

/// ゲームを操作するボタンのセットアップを行う関数
fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    info_once!("setup");

    // ボタンフィールドを生成
    let init_pos = Vec2::new(BUTTON_MARGIN + BUTTON_SIZE, BUTTON_FIELD_SIZE.y / 2.0 - BUTTON_SIZE / 2.0);
    commands
        .spawn(KeyButton::from_board())
        // 左側の左キーを作成
        .with_children(|parent| {
            parent
                .spawn((KeyButton::from_left_button(init_pos.x - BUTTON_SIZE, init_pos.y), MoveLeftButton))
                .with_children(|parent| {
                    let image = asset_server.load(PATH_IMAGE_ANGLE_LEFT);
                    parent.spawn(KeyButton::from_icon(image));
                });
        })
        // 左側の右キーを作成
        .with_children(|parent| {
            parent
                .spawn((KeyButton::from_left_button(init_pos.x + BUTTON_SIZE, init_pos.y), MoveRightButton))
                .with_children(|parent| {
                    let image = asset_server.load(PATH_IMAGE_ANGLE_RIGHT);
                    parent.spawn(KeyButton::from_icon(image));
                });
        })
        // 左側の上キーを作成
        .with_children(|parent| {
            parent
                .spawn((KeyButton::from_left_button(init_pos.x, init_pos.y - BUTTON_SIZE), RotateRightButton))
                .with_children(|parent| {
                    let image = asset_server.load(PATH_IMAGE_ANGLE_UP);
                    parent.spawn(KeyButton::from_icon(image));
                });
        })
        // 左側の下キーを作成
        .with_children(|parent| {
            parent
                .spawn((KeyButton::from_left_button(init_pos.x, init_pos.y + BUTTON_SIZE), FallButton))
                .with_children(|parent| {
                    let image = asset_server.load(PATH_IMAGE_ANGLE_DOWN);
                    parent.spawn(KeyButton::from_icon(image));
                });
        })
        // 右側の左キーを作成
        .with_children(|parent| {
            parent
                .spawn((KeyButton::from_right_button(init_pos.x + BUTTON_SIZE, init_pos.y), RotateLeftButton))
                .with_children(|parent| {
                    let image = asset_server.load(PATH_IMAGE_ANGLE_LEFT);
                    parent.spawn(KeyButton::from_icon(image));
                });
        })
        // 右側の右キーを作成
        .with_children(|parent| {
            parent
                .spawn((KeyButton::from_right_button(init_pos.x - BUTTON_SIZE, init_pos.y), RotateRightButton))
                .with_children(|parent| {
                    let image = asset_server.load(PATH_IMAGE_ANGLE_RIGHT);
                    parent.spawn(KeyButton::from_icon(image));
                });
        })
        // 右側の上キーを作成
        .with_children(|parent| {
            parent
                .spawn((KeyButton::from_right_button(init_pos.x, init_pos.y - BUTTON_SIZE), HoldButton))
                .with_children(|parent| {
                    let image = asset_server.load(PATH_IMAGE_HOLD);
                    parent.spawn(KeyButton::from_icon(image));
                });
        })
        // 右側の下キーを作成
        .with_children(|parent| {
            parent
                .spawn((KeyButton::from_right_button(init_pos.x, init_pos.y + BUTTON_SIZE), FixButton))
                .with_children(|parent| {
                    let image = asset_server.load(PATH_IMAGE_FALL);
                    parent.spawn(KeyButton::from_icon(image));
                });
        });
}


pub struct ButtonPlugin;

impl Plugin for ButtonPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::InGame), setup)
        ;
    }
}
