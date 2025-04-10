use bevy::prelude::*;

use crate::{
    WINDOW_SIZE,
    PATH_FONT,
    PATH_IMAGE_RETRY,
    AppState,
};

const BOARD_WIDTH: Val = Val::Px(360.0);
const BOARD_HEIGHT: Val = Val::Px(270.0);
const BOARD_LEFT: Val = Val::Px(WINDOW_SIZE.x / 2.0 - 360.0 / 2.0);
const BOARD_TOP: Val = Val::Px(WINDOW_SIZE.y / 2.0 - 270.0 / 2.0);
const BOARD_PADDING: Val = Val::Px(16.0);
const BOARD_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);

const GAMEOVER_TEXT: &str = "ゲームオーバー";
const GAMEOVER_FONT_SIZE: f32 = 24.0;
const GAMEOVER_COLOR: Color = Color::srgb(0.1, 0.1, 0.1);

const RETRY_SIZE: Vec2 = Vec2::new(24.0, 24.0);
const RETRY_BACKGROUND_COLOR_HOVER: Color = Color::srgb(0.8, 0.8, 0.8);

const BORDER_SIZE: Val = Val::Px(4.0);
const BORDER_COLOR: Color = Color::srgb(0.5, 0.5, 1.0);
const BORDER_RADIUS: Val = Val::Px(10.0);

#[derive(Component)]
struct Gameover;

impl Gameover {
    fn from_root() -> (Self, Node) {
        (
            Self,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..Default::default()
            }
        )
    }

    fn from_board() -> (Self, Node, BackgroundColor, BorderColor, BorderRadius) {
        (
            Self,
            Node {
                width: BOARD_WIDTH,
                height: BOARD_HEIGHT,
                border: UiRect::all(BORDER_SIZE),
                position_type: PositionType::Absolute,
                left: BOARD_LEFT,
                top: BOARD_TOP,
                padding: UiRect::all(BOARD_PADDING),
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                ..Default::default()
            },
            BackgroundColor(BOARD_COLOR),
            BorderColor(BORDER_COLOR),
            BorderRadius::all(BORDER_RADIUS),
        )
    }

    fn from_text(font: Handle<Font>) -> (Self, Text, TextFont, TextColor) {
        (
            Self,
            Text::new(GAMEOVER_TEXT),
            TextFont {
                font: font.clone(),
                font_size: GAMEOVER_FONT_SIZE,
                ..Default::default()
            },
            TextColor(GAMEOVER_COLOR),
        )
    }

    fn from_retry() -> (Self, Node, BorderColor, BorderRadius, Button) {
        (
            Self,
            Node {
                width: Val::Px(RETRY_SIZE.x * 2.0),
                height: Val::Px(RETRY_SIZE.y * 2.0),
                border: UiRect::all(BORDER_SIZE),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            BorderColor(BORDER_COLOR),
            BorderRadius::all(BORDER_RADIUS),
            Button,
        )
    }

    fn from_retry_icon(image: Handle<Image>) -> (Self, ImageNode, Node) {
        (
            Self,
            ImageNode::new(image.clone()),
            Node {
                width: Val::Px(RETRY_SIZE.x),
                height: Val::Px(RETRY_SIZE.y),
                ..Default::default()
            },
        )
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let font = asset_server.load(PATH_FONT);
    let image = asset_server.load(PATH_IMAGE_RETRY);

    // ノードツリー
    // root
    // └── board
    //     ├── gameover text
    //     └── retry
    //         └── icon
    commands
        // ルートを生成
        .spawn(Gameover::from_root())
        .with_children(|parent| {
            // ボードを生成
            parent.spawn(Gameover::from_board())
                .with_children(|parent| {
                    // ゲームオーバーテキストを生成
                    parent.spawn(Gameover::from_text(font));
                })
                .with_children(|parent| {
                    // リトライを生成
                    parent.spawn(Gameover::from_retry())
                        .with_children(|parent| {
                            // リトライアイコンを生成
                            parent.spawn(Gameover::from_retry_icon(image));
                        });
                });
        });
}

fn update(
) {
}

pub struct GameoverPlugin;

impl Plugin for GameoverPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::Gameover), setup)
            .add_systems(Update, update.run_if(in_state(AppState::Gameover)))
        ;
    }
}
