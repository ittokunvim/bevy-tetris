use bevy::prelude::*;

use crate::{
    WINDOW_SIZE,
    PATH_FONT,
    PATH_IMAGE_HOUSE,
    PATH_IMAGE_RETRY,
    AppState,
};

const ROOT_WIDTH: Val = Val::Percent(100.0);
const ROOT_HEIGHT: Val = Val::Percent(100.0);

const BOARD_SIZE: Vec2 = Vec2::new(360.0, 270.0);
const BOARD_LEFT: Val = Val::Px(WINDOW_SIZE.x / 2.0 - BOARD_SIZE.x / 2.0);
const BOARD_TOP: Val = Val::Px(WINDOW_SIZE.y / 2.0 - BOARD_SIZE.y / 2.0);
const BOARD_PADDING: Val = Val::Px(16.0);
const BOARD_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);

const TITLE_TEXT: &str = "ゲームオーバー";
const TITLE_FONT_SIZE: f32 = 24.0;
const TITLE_COLOR: Color = Color::srgb(0.1, 0.1, 0.1);

const LIST_WIDTH: Val = Val::Px(BOARD_SIZE.x);
const LIST_HEIGHT: Val = Val::Px(48.0);

const ICON_SIZE: Vec2 = Vec2::new(24.0, 24.0);
const BUTTON_WIDTH: Val = Val::Px(ICON_SIZE.x * 2.0);
const BUTTON_HEIGHT: Val = Val::Px(ICON_SIZE.y * 2.0);

const HOUSE_COLOR_HOVER: Color = Color::srgb(0.4, 0.8, 0.4);
const RETRY_COLOR_HOVER: Color = Color::srgb(0.4, 0.4, 0.8);

const BORDER_SIZE: Val = Val::Px(4.0);
const BORDER_COLOR: Color = Color::srgb(0.5, 0.5, 1.0);
const BORDER_RADIUS: Val = Val::Px(10.0);

#[derive(Component)]
struct Gameover;

#[derive(Component)]
struct Home;

#[derive(Component)]
struct Retry;

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
                width: ROOT_WIDTH,
                height: ROOT_HEIGHT,
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
                width: Val::Px(BOARD_SIZE.x),
                height: Val::Px(BOARD_SIZE.y),
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
    /// ゲームオーバー画面に表示するボタンの配置を決めるノード
    ///
    /// Returns:
    /// * `Self`: Gameoverのインスタンス。
    /// * `Node`: ボタンの配置を決めるノード
    fn from_button_list() -> (Self, Node) {
        (
            Self,
            Node {
                width: LIST_WIDTH,
                height: LIST_HEIGHT,
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::SpaceAround,
                align_items: AlignItems::Center,
                ..Default::default()
            }
        )
    }
    /// ゲームオーバー画面に表示するボタンを生成します。
    ///
    /// Returns:
    /// * `Self`: Gameoverのインスタンス。
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
    /// ゲームオーバー画面に表示するアイコンを生成します。
    ///
    /// Params:
    /// * `image`: アイコン画像
    ///
    /// Returns:
    /// * `Self`: Gameoverのインスタンス。
    /// * `ImageNode`: 画像のノード
    /// * `Node`: アイコンのサイズ、レイアウトを表すノード。
    fn from_icon(image: Handle<Image>) -> (Self, ImageNode, Node) {
        (
            Self,
            ImageNode::new(image.clone()),
            Node {
                width: Val::Px(ICON_SIZE.x),
                height: Val::Px(ICON_SIZE.y),
                ..Default::default()
            },
        )
    }
}

/// 構造:
/// * root
///   * board
///     * gameover text
///     * button list
///       * house button
///         * icon
///       * retry button
///         * icon
fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let font = asset_server.load(PATH_FONT);
    let house_image = asset_server.load(PATH_IMAGE_HOUSE);
    let retry_image = asset_server.load(PATH_IMAGE_RETRY);

    commands
        // ルートノードを生成
        .spawn(Gameover::from_root())
        .with_children(|parent| {
            parent
                // ボードノードを生成
                .spawn(Gameover::from_board())
                .with_children(|parent| {
                    // ゲームオーバーテキストノードを生成
                    parent.spawn(Gameover::from_title(font));
                })
                .with_children(|parent| {
                    parent
                        // ボタンリストを生成
                        .spawn(Gameover::from_button_list())
                        .with_children(|parent| {
                            parent
                                // ホームボタンノードを生成
                                .spawn((Gameover::from_button(), Home))
                                .with_children(|parent| {
                                    // ホームアイコンノードを生成
                                    parent.spawn(Gameover::from_icon(house_image.clone()));
                                });
                        })
                        .with_children(|parent| {
                            parent
                                // リトライボタンノードを生成
                                .spawn((Gameover::from_button(), Retry))
                                .with_children(|parent| {
                                    // リトライアイコンノードを生成
                                    parent.spawn(Gameover::from_icon(retry_image.clone()));
                                });
                        });
                });
        });
}

fn house_button_system(
    mut interaction_query: Query<
    (&Interaction, &mut BackgroundColor),
    (Changed<Interaction>, (With<Home>, With<Button>)),
    >,
    mut next_state: ResMut<NextState<AppState>>,
) {
    // 全てのインタラクション状態を持つホームボタンに対して処理を行う
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            // ボタンが押された時の処理
            Interaction::Pressed => {
                next_state.set(AppState::Mainmenu);
            }
            // ボタンがホバーされた時の処理
            Interaction::Hovered => {
                *color = HOUSE_COLOR_HOVER.into();
            }
            // ボタンに何もされていない時の処理
            Interaction::None => {
                *color = BOARD_COLOR.into();
            }
        }
    }
}

fn retry_button_system(
    mut interaction_query: Query<
    (&Interaction, &mut BackgroundColor),
    (Changed<Interaction>, (With<Retry>, With<Button>)),
    >,
    mut next_state: ResMut<NextState<AppState>>,
) {
    // 全てのインタラクション状態を持つリトライボタンに対して処理を行う
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            // ボタンが押された時の処理
            Interaction::Pressed => {
                next_state.set(AppState::InGame);
            }
            // ボタンがホバーされた時の処理
            Interaction::Hovered => {
                *color = RETRY_COLOR_HOVER.into();
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
            .add_systems(Update, (
                retry_button_system,
                house_button_system,
            ).run_if(in_state(AppState::Gameover)))
            .add_systems(OnExit(AppState::Gameover), despawn)
        ;
    }
}
