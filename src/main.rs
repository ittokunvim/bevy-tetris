use bevy::{
    prelude::*,
    log::LogPlugin,
    time::Stopwatch,
};

mod block;
mod bgm;
mod blockdata;
mod field;
mod key;

mod mainmenu;
mod gameover;

const GAMETITLE: &str = "テトリス";
const WINDOW_SIZE: Vec2 = Vec2::new(640.0, 480.0);
const BACKGROUND_COLOR: Color = Color::srgb(0.27, 0.29, 0.45);
const LOG_FILTER: &str = "info,wgpu_core=warn,wgpu_hal=warn,ittokunvim_tetris=debug";
const PATH_FONT: &str = "fonts/misaki_gothic.ttf";
const PATH_IMAGE_HOUSE: &str = "images/house-dark.png";
const PATH_IMAGE_RETRY: &str = "images/rotate-left-dark.png";
const PATH_SOUND_BGM: &str = "bevy-tetris/bgm.ogg";

const GRID_SIZE: f32 = 20.0;
const BLOCK_FALL_SPEED: f32 = 0.5;
const BLOCK_MOVE_SPEED: f32 = 0.25;
const FIELD_SIZE: Vec2 = Vec2::new(10.0 * GRID_SIZE, 20.0 * GRID_SIZE);
const FIELD_POSITION: Vec3 = Vec3::new(0.0, 0.0, -10.0);

#[derive(Event)]
struct MoveEvent(Direction);

#[derive(Event)]
struct RotationEvent(Direction);

#[derive(Event, Default)]
struct HardDropEvent;

#[derive(Event, Default)]
struct SpawnEvent;

#[derive(Event, Default)]
struct FixEvent;

#[derive(Copy, Clone, PartialEq, Debug)]
enum Direction {
    Left,
    Right,
    Bottom,
}

#[derive(Resource, Deref, DerefMut)]
struct FallingTimer(Timer);

#[derive(Resource, Deref, DerefMut)]
struct MoveLeftTimer(Stopwatch);

#[derive(Resource, Deref, DerefMut)]
struct MoveBottomTimer(Stopwatch);

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
enum AppState {
    #[default]
    Mainmenu,
    InGame,
    Gameover,
}

impl FallingTimer {
    fn new() -> Self {
        Self(Timer::from_seconds(BLOCK_FALL_SPEED, TimerMode::Repeating))
    }
}

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
        )
        .init_state::<AppState>()
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .insert_resource(Time::<Fixed>::from_seconds(1.0 / 60.0))
        .add_event::<MoveEvent>()
        .add_event::<RotationEvent>()
        .add_event::<HardDropEvent>()
        .add_event::<SpawnEvent>()
        .add_event::<FixEvent>()
        .insert_resource(FallingTimer::new())
        .insert_resource(MoveLeftTimer(Stopwatch::new()))
        .insert_resource(MoveBottomTimer(Stopwatch::new()))
        .add_plugins(field::FieldPlugin)
        .add_plugins(key::KeyPlugin)
        .add_plugins(block::BlockPlugin)
        .add_plugins(bgm::BgmPlugin)
        .add_plugins(mainmenu::MainmenuPlugin)
        .add_plugins(gameover::GameoverPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    info_once!("setup");

    commands.spawn(Camera2d::default());
}

