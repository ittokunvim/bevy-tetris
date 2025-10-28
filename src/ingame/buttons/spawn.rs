use bevy::prelude::*;

use crate::{
    WINDOW_SIZE,
    PATH_IMAGE_ANGLE_DOWN,
    PATH_IMAGE_ANGLE_LEFT,
    PATH_IMAGE_ANGLE_RIGHT,
    PATH_IMAGE_ANGLE_UP,
    PATH_IMAGE_FALL,
    PATH_IMAGE_HOLD,
};

use super::{
    KeyButton,
    MoveLeftButton,
    MoveRightButton,
    MoveBottomButton,
    RotateLeftButton,
    RotateRightButton,
    HoldButton,
    HarddropButton,
};

const BUTTON_FIELD_SIZE: Vec2 = Vec2::new(WINDOW_SIZE.x, WINDOW_SIZE.y / 4.0);
const BUTTON_FIELD_COLOR: Color = Color::srgb(0.14, 0.16, 0.23);
const BUTTON_SIZE: f32 = 45.0;
const BUTTON_COLOR: Color = Color::srgb(0.90, 0.90, 0.90);
const BUTTON_MARGIN: f32 = 5.0;
const ICON_SIZE: f32 = 35.0;

impl KeyButton {
    /// ゲーム画面に配置するボタンの背景を生成します
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
            BackgroundColor(BUTTON_FIELD_COLOR),
        )
    }

    /// 左側に配置するボタンを生成します
    fn from_left_button(pos: Vec2) -> (
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
                left: Val::Px(pos.x),
                top: Val::Px(pos.y),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            BorderColor::all(BUTTON_COLOR),
            BorderRadius::MAX,
            BackgroundColor(BUTTON_COLOR),
        )
    }

    /// 右側に配置するボタンを生成します
    fn from_right_button(pos: Vec2) -> (
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
                right: Val::Px(pos.x),
                top: Val::Px(pos.y),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            BorderColor::all(BUTTON_COLOR),
            BorderRadius::MAX,
            BackgroundColor(BUTTON_COLOR),
        )
    }

    /// ボタンの中に配置するアイコンを生成します
    fn from_icon(image: Handle<Image>) -> (Self, ImageNode, Node) {
        (
            Self,
            ImageNode::new(image),
            Node {
                width: Val::Px(ICON_SIZE),
                height: Val::Px(ICON_SIZE),
                ..Default::default()
            },
        )
    }
}

/// ゲームを操作するボタンのセットアップを行う関数
pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    info_once!("setup");

    // ボタンフィールドを生成
    let init_pos = Vec2::new(
        BUTTON_MARGIN + BUTTON_SIZE,
        BUTTON_FIELD_SIZE.y / 2.0 - BUTTON_SIZE / 2.0,
    );
    commands
        // ボタンフィールドを生成
        .spawn(KeyButton::from_board())
        // 左側の左キーを作成
        .with_children(|parent| {
            let pos = init_pos.with_x(init_pos.x - BUTTON_SIZE);
            parent
                .spawn((KeyButton::from_left_button(pos), MoveLeftButton))
                .with_children(|parent| {
                    let image = asset_server.load(PATH_IMAGE_ANGLE_LEFT);
                    parent.spawn(KeyButton::from_icon(image));
                });
        })
        // 左側の右キーを作成
        .with_children(|parent| {
            let pos = init_pos.with_x(init_pos.x + BUTTON_SIZE);
            parent
                .spawn((KeyButton::from_left_button(pos), MoveRightButton))
                .with_children(|parent| {
                    let image = asset_server.load(PATH_IMAGE_ANGLE_RIGHT);
                    parent.spawn(KeyButton::from_icon(image));
                });
        })
        // 左側の上キーを作成
        .with_children(|parent| {
            let pos = init_pos.with_y(init_pos.y - BUTTON_SIZE);
            parent
                .spawn((KeyButton::from_left_button(pos), RotateRightButton))
                .with_children(|parent| {
                    let image = asset_server.load(PATH_IMAGE_ANGLE_UP);
                    parent.spawn(KeyButton::from_icon(image));
                });
        })
        // 左側の下キーを作成
        .with_children(|parent| {
            let pos = init_pos.with_y(init_pos.y + BUTTON_SIZE);
            parent
                .spawn((KeyButton::from_left_button(pos), MoveBottomButton))
                .with_children(|parent| {
                    let image = asset_server.load(PATH_IMAGE_ANGLE_DOWN);
                    parent.spawn(KeyButton::from_icon(image));
                });
        })
        // 右側の左キーを作成
        .with_children(|parent| {
            let pos = init_pos.with_x(init_pos.x + BUTTON_SIZE);
            parent
                .spawn((KeyButton::from_right_button(pos), RotateLeftButton))
                .with_children(|parent| {
                    let image = asset_server.load(PATH_IMAGE_ANGLE_LEFT);
                    parent.spawn(KeyButton::from_icon(image));
                });
        })
        // 右側の右キーを作成
        .with_children(|parent| {
            let pos = init_pos.with_x(init_pos.x - BUTTON_SIZE);
            parent
                .spawn((KeyButton::from_right_button(pos), RotateRightButton))
                .with_children(|parent| {
                    let image = asset_server.load(PATH_IMAGE_ANGLE_RIGHT);
                    parent.spawn(KeyButton::from_icon(image));
                });
        })
        // 右側の上キーを作成
        .with_children(|parent| {
            let pos = init_pos.with_y(init_pos.y - BUTTON_SIZE);
            parent
                .spawn((KeyButton::from_right_button(pos), HoldButton))
                .with_children(|parent| {
                    let image = asset_server.load(PATH_IMAGE_HOLD);
                    parent.spawn(KeyButton::from_icon(image));
                });
        })
        // 右側の下キーを作成
        .with_children(|parent| {
            let pos = init_pos.with_y(init_pos.y + BUTTON_SIZE);
            parent
                .spawn((KeyButton::from_right_button(pos), HarddropButton))
                .with_children(|parent| {
                    let image = asset_server.load(PATH_IMAGE_FALL);
                    parent.spawn(KeyButton::from_icon(image));
                });
        });
}

