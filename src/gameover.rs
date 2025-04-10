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
    /// ゲームオーバー画面のルートノードを生成します
    ///
    /// Returns:
    /// * `Self`: Gameoverのインスタンス。
    /// * `Node`: 幅と高さが100%のルートノード。
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

    /// ゲームオーバー画面の背景を生成します。
    ///
    /// Returns:
    /// * `Self`: Gameoverのインスタンス。
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

    /// ゲームオーバーメッセージを表示するテキストを生成します。
    ///
    /// Params:
    /// * `font`: テキストに使用するフォント
    ///
    /// Returns:
    /// * `Self`: Gameoverのインスタンス。
    /// * `Text`: ゲームオーバーメッセージのテキスト。
    /// * `TextFont`: フォントスタイル。
    /// * `TextColor`: テキストの色
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

    /// ゲームオーバー画面に表示する「リトライ」ボタンを生成します。
    ///
    /// Returns:
    /// * `Self`: Gameoverのインスタンス。
    /// * `Node`: リトライボタンを表すノード。
    /// * `BorderColor`: ボーダーの色
    /// * `BorderRadius`: ボーダーのラディウス
    /// * `Button`: ボタンコンポーネント
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

    /// ゲームオーバー画面に表示するリトライアイコンを生成します。
    ///
    /// Params:
    /// * `image`: リトライアイコン
    ///
    /// Returns:
    /// * `Self`: Gameoverのインスタンス。
    /// * `ImageNode`: 画像のノード
    /// * `Node`: リトライアイコンのサイズ、レイアウトを表すノード。
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

/// 構造:
/// * root
///   * board
///     * gameover text
///     * retry
///       * icon
fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let font = asset_server.load(PATH_FONT);
    let image = asset_server.load(PATH_IMAGE_RETRY);

    commands
        // ルートノードを生成
        .spawn(Gameover::from_root())
        .with_children(|parent| {
            // ボードノードを生成
            parent.spawn(Gameover::from_board())
                .with_children(|parent| {
                    // ゲームオーバーテキストノードを生成
                    parent.spawn(Gameover::from_text(font));
                })
                .with_children(|parent| {
                    // リトライボタンノードを生成
                    parent.spawn(Gameover::from_retry())
                        .with_children(|parent| {
                            // リトライアイコンノードを生成
                            parent.spawn(Gameover::from_retry_icon(image));
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
                *color = RETRY_BACKGROUND_COLOR_HOVER.into();
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
    query: Query<Entity, With<Gameover>>,
) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

pub struct GameoverPlugin;

impl Plugin for GameoverPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::Gameover), setup)
            .add_systems(Update, update.run_if(in_state(AppState::Gameover)))
            .add_systems(OnExit(AppState::Gameover), despawn)
        ;
    }
}
