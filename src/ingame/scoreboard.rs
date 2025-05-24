use bevy::prelude::*;

use crate::{
    GRID_SIZE_HALF,
    PATH_FONT,
    AppState,
};
use super::{
    FIELD_SIZE,
    FIELD_POSITION,
};

const BOARD_SIZE: Vec2 = Vec2::new(
    GRID_SIZE_HALF * 6.0,
    GRID_SIZE_HALF * 10.0,
);
const BOARD_POSITION: Vec3 = Vec3::new(
    FIELD_POSITION.x - FIELD_SIZE.x / 2.0 - BOARD_SIZE.x / 2.0,
    FIELD_POSITION.y - FIELD_SIZE.y / 2.0 + BOARD_SIZE.y / 2.0,
    0.0,
);
const BOARD_COLOR: Color = Color::srgb(0.16, 0.18, 0.26);

const TITLE_TEXT: &str = "SCORE";
const TITLE_POSITION: Vec3 = Vec3::new(
    BOARD_POSITION.x, 
    BOARD_POSITION.y + BOARD_SIZE.y / 2.0 - TEXT_SIZE / 2.0 - GRID_SIZE_HALF * 1.75,
    10.0,
);

const SCORE_POSITION: Vec3 = Vec3::new(
    BOARD_POSITION.x, 
    BOARD_POSITION.y - BOARD_SIZE.y / 2.0 + TEXT_SIZE / 2.0 + GRID_SIZE_HALF * 1.75,
    10.0,
);

const TEXT_SIZE: f32 = 20.0;

/// スコアの点数を管理するリソース
#[derive(Resource, Debug, Deref, DerefMut)]
pub struct Score(pub usize);

#[derive(Component)]
struct Scoreboard;

/// スコアの点数の更新をするためのコンポーネント
#[derive(Component)]
struct ScoreText;

/// スコアボードのセットアップを行う関数
fn setup(
    mut commands: Commands,
    score: Res<Score>,
    asset_server: Res<AssetServer>,
) {
    info_once!("setup");

    let font = asset_server.load(PATH_FONT);

    // ボードを生成する
    commands.spawn((
        Sprite::from_color(BOARD_COLOR, BOARD_SIZE),
        Transform::from_translation(BOARD_POSITION),
        Scoreboard,
    ));

    // タイトルを生成する
    commands.spawn((
        Text2d::new(TITLE_TEXT),
        TextFont {
            font: font.clone(),
            font_size: TEXT_SIZE,
            ..Default::default()
        },
        Transform::from_translation(TITLE_POSITION),
        Scoreboard,
    ));

    // スコアを生成する
    commands.spawn((
        Text2d::new(score.0.to_string()),
        TextFont {
            font: font.clone(),
            font_size: TEXT_SIZE,
            ..Default::default()
        },
        Transform::from_translation(SCORE_POSITION),
        Scoreboard,
        ScoreText,
    ));
}

/// スコアを更新する関数
fn update_score(
    mut query: Query<&mut Text2d, With<ScoreText>>,
    score: Res<Score>,
) {
    info_once!("update");

    let mut span = query.single_mut();
    **span = score.0.to_string();
}

pub struct ScoreboardPlugin;

impl Plugin for ScoreboardPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(Score(0))
            .add_systems(OnEnter(AppState::InGame), setup)
            .add_systems(Update, update_score.run_if(in_state(AppState::InGame)))
        ;
    }
}
