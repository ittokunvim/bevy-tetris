use bevy::{
    prelude::*,
    log::LogPlugin,
    asset::AssetMetaCheck,
};

mod mainmenu;
mod ingame;
mod gameover;

mod sound;

const GAMETITLE: &str = "いっとくテトリス";
const WINDOW_SIZE: Vec2 = Vec2::new(640.0, 480.0);
const BACKGROUND_COLOR: Color = Color::srgb(0.27, 0.29, 0.45);
const LOG_FILTER: &str = "info,wgpu_core=warn,wgpu_hal=warn,ittoku_tetris=debug";
const PATH_FONT: &str = "fonts/misaki_gothic.ttf";
const PATH_IMAGE_HOUSE: &str = "images/house-dark.png";
const PATH_IMAGE_RETRY: &str = "images/rotate-left-dark.png";
const PATH_SOUND_BGM: &str = "ittoku-tetris/bgm.ogg";
const PATH_SOUND_CLICK: &str = "sounds/click.ogg";

const GRID_SIZE: f32 = 20.0;
const GRID_SIZE_HALF: f32 = GRID_SIZE / 2.0;

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash, Resource)]
enum AppState {
    #[default]
    Mainmenu,
    InGame,
    Gameover,
}

/// スコアの点数を管理するリソース
#[derive(Resource, Debug, Deref, DerefMut)]
pub struct Score(pub usize);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    resolution: WINDOW_SIZE.into(),
                    title: GAMETITLE.to_string(),
                    ..Default::default()
                }),
                ..Default::default()
            })
            .set(LogPlugin {
                filter: LOG_FILTER.into(),
                level: bevy::log::Level::DEBUG,
                ..Default::default()
            })
            .set(AssetPlugin {
                meta_check: AssetMetaCheck::Never,
                ..Default::default()
            })
        )
        .init_state::<AppState>()
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .insert_resource(Time::<Fixed>::from_seconds(1.0 / 60.0))
        .insert_resource(Score(0))
        .add_plugins(mainmenu::MainmenuPlugin)
        .add_plugins(ingame::IngamePlugin)
        .add_plugins(gameover::GameoverPlugin)
        .add_plugins(sound::SoundPlugin)
        .add_systems(Startup, setup)
        .add_systems(OnExit(AppState::Gameover), reset_score)
        .run();
}

fn setup(mut commands: Commands) {
    info_once!("setup");

    commands.spawn(Camera2d::default());
}

fn reset_score(mut score: ResMut<Score>) {
    info_once!("reset_score");

    **score = 0;
}
