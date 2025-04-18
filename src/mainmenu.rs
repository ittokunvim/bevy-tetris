use bevy::prelude::*;

use crate::{
    WINDOW_SIZE,
    GAMETITLE,
    PATH_FONT,
    AppState,
};

const ROOT_WIDTH: Val = Val::Percent(100.0);
const ROOT_HEIGHT: Val = Val::Percent(100.0);

const BOARD_SIZE: Vec2 = Vec2::new(360.0, 270.0);
const BOARD_WIDTH: Val = Val::Px(BOARD_SIZE.x);
const BOARD_HEIGHT: Val = Val::Px(BOARD_SIZE.y);
const BOARD_LEFT: Val = Val::Px(WINDOW_SIZE.x / 2.0 - BOARD_SIZE.x / 2.0);
const BOARD_TOP: Val = Val::Px(WINDOW_SIZE.y / 2.0 - BOARD_SIZE.y / 2.0);
const BOARD_PADDING: Val = Val::Px(16.0);
const BOARD_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);

const TITLE_TEXT: &str = GAMETITLE;
const TITLE_FONT_SIZE: f32 = 24.0;
const TITLE_COLOR: Color = Color::srgb(0.1, 0.1, 0.1);

const BUTTON_WIDTH: Val = Val::Px(128.0);
const BUTTON_HEIGHT: Val = Val::Px(48.0);

const PLAY_TEXT: &str = "はじめる";
const PLAY_FONT_SIZE: f32 = 20.0;
const PLAY_COLOR: Color = Color::srgb(0.1, 0.1, 0.1);
const PLAY_BACKGROUND_COLOR_HOVER: Color = Color::srgb(0.4, 0.8, 0.4);

const BORDER_SIZE: Val = Val::Px(4.0);
const BORDER_COLOR: Color = Color::srgb(0.5, 0.5, 1.0);
const BORDER_RADIUS: Val = Val::Px(10.0);

#[derive(Component)]
struct Mainmenu;

impl Mainmenu {
    /// メインメニュー画面のルートノードを生成します
    ///
    /// Returns:
    /// * `Self`: Mainmenuのインスタンス。
    /// * `Node`: 幅と高さが100%のルートノード。
    fn from_root() -> (Self, Node) {
        (
            Self,
            Node {
                width: ROOT_WIDTH,
                height: ROOT_HEIGHT,
                ..Default::default()
            }
        )
    }
    /// メインメニュー画面の背景を生成します。
    ///
    /// Returns:
    /// * `Self`: Mainmenuのインスタンス。
    /// * `Node`: 背景のサイズ、場所、並び方などが定義されたノード。 
    /// * `BackgroundColor`: 背景色
    /// * `BorderColor`: ボーダーの色
    /// * `BorderRadius`: ボーダーのラディウス
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
    /// ゲームタイトルを表示するタイトルを生成します。
    ///
    /// Params:
    /// * `font`: テキストに使用するフォント
    ///
    /// Returns:
    /// * `Self`: Mainmenuのインスタンス。
    /// * `Text`: ゲームオーバーメッセージのテキスト。
    /// * `TextFont`: フォントスタイル。
    /// * `TextColor`: テキストの色
    fn from_title(font: Handle<Font>) -> (Self, Text, TextFont, TextColor) {
        (
            Self,
            Text::new(TITLE_TEXT),
            TextFont {
                font: font.clone(),
                font_size: TITLE_FONT_SIZE,
                ..Default::default()
            },
            TextColor(TITLE_COLOR),
         )
    }
    /// メインメニュー画面に表示するボタンを生成します。
    ///
    /// Returns:
    /// * `Self`: Mainmenuのインスタンス。
    /// * `Node`: リトライボタンを表すノード。
    /// * `BorderColor`: ボーダーの色
    /// * `BorderRadius`: ボーダーのラディウス
    /// * `Button`: ボタンコンポーネント
    fn from_button() -> (Self, Node, BorderColor, BorderRadius, Button) {
        (
            Self,
            Node {
                width: BUTTON_WIDTH,
                height: BUTTON_HEIGHT,
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
    /// ボタンのテキストを生成します。
    ///
    /// Params:
    /// * `font`: テキストに使用するフォント
    ///
    /// Returns:
    /// * `Self`: Mainmenuのインスタンス。
    /// * `Text`: ゲームオーバーメッセージのテキスト。
    /// * `TextFont`: フォントスタイル。
    /// * `TextColor`: テキストの色
    fn from_text(font: Handle<Font>) -> (Self, Text, TextFont, TextColor) {
        (
            Self,
            Text::new(PLAY_TEXT),
            TextFont {
                font: font.clone(),
                font_size: PLAY_FONT_SIZE,
                ..Default::default()
            },
            TextColor(PLAY_COLOR),
        )
    }
}

/// 構造:
/// * root
///   * board
///     * mainmenu text
///     * play button
///       * button text
fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let font = asset_server.load(PATH_FONT);

    commands
        // ルートノードを作成
        .spawn(Mainmenu::from_root())
        .with_children(|parent| {
            parent
                // ボードノードを作成
                .spawn(Mainmenu::from_board())
                .with_children(|parent| {
                    // タイトルノードを作成
                    parent.spawn(Mainmenu::from_title(font.clone()));
                })
                .with_children(|parent| {
                    // プレイボタンノードを作成
                    parent
                        .spawn(Mainmenu::from_button())
                        .with_children(|parent| {
                            // ボタンテキストノードを作成
                            parent.spawn(Mainmenu::from_text(font.clone()));
                        });
                });
        });
}

fn update(
    mut interaction_query: Query<
    (&Interaction, &mut BackgroundColor),
    (Changed<Interaction>, With<Button>),
    >,
    mut next_state: ResMut<NextState<AppState>>,
) {
    // 全てのインタラクション状態を持つボタンに対して処理を行う
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            // ボタンが押された時の処理
            Interaction::Pressed => {
                next_state.set(AppState::InGame);
            }
            // ボタンがホバーされた時の処理
            Interaction::Hovered => {
                *color = PLAY_BACKGROUND_COLOR_HOVER.into();
            }
            // ボタンに何もされていない時の処理
            Interaction::None => {
                *color = BOARD_COLOR.into();
            }
        }
    }
}

fn despawn(
    mut commands: Commands,
    query: Query<Entity, With<Mainmenu>>,
) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

pub struct MainmenuPlugin;

impl Plugin for MainmenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::Mainmenu), setup)
            .add_systems(Update, update.run_if(in_state(AppState::Mainmenu)))
            .add_systems(OnExit(AppState::Mainmenu), despawn)
        ;
    }
}
