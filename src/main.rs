use bevy::prelude::*;

mod block;
mod blockdata;
mod field;
mod key;

mod mainmenu;
mod gameover;

const GAMETITLE: &str = "テトリス";
const WINDOW_SIZE: Vec2 = Vec2::new(640.0, 480.0);
const BACKGROUND_COLOR: Color = Color::srgb(0.1, 0.1, 0.1);
const PATH_FONT: &str = "fonts/misaki_gothic.ttf";
const PATH_IMAGE_RETRY: &str = "images/retry.png";
const PATH_SOUND_BGM: &str = "bevy-tetris/bgm.ogg";

const GRID_SIZE: f32 = 20.0;
const BLOCK_SPEED: f32 = 0.5;
const FIELD_SIZE: Vec2 = Vec2::new(10.0 * GRID_SIZE, 20.0 * GRID_SIZE);
const FIELD_POSITION: Vec3 = Vec3::new(0.0, 0.0, -10.0);

#[derive(Event)]
struct MoveEvent(Direction);

#[derive(Event)]
struct RotationEvent(Direction);

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

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
enum AppState {
    #[default]
    Mainmenu,
    InGame,
    Gameover,
}

impl FallingTimer {
    fn new() -> Self {
        Self(Timer::from_seconds(BLOCK_SPEED, TimerMode::Repeating))
    }

    fn update_timer(seconds: f32) -> Timer {
        Timer::from_seconds(seconds, TimerMode::Repeating)
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
        )
        .init_state::<AppState>()
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .insert_resource(Time::<Fixed>::from_seconds(1.0 / 60.0))
        .add_event::<MoveEvent>()
        .add_event::<RotationEvent>()
        .add_event::<SpawnEvent>()
        .add_event::<FixEvent>()
        .insert_resource(FallingTimer::new())
        .add_plugins(field::FieldPlugin)
        .add_plugins(key::KeyPlugin)
        .add_plugins(block::BlockPlugin)
        .add_plugins(mainmenu::MainmenuPlugin)
        .add_plugins(gameover::GameoverPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // camera
    commands.spawn(Camera2d::default());
    // bgm
    let sound = AudioPlayer::new(asset_server.load(PATH_SOUND_BGM));
    let settings = PlaybackSettings::LOOP;
    commands.spawn((sound, settings));
}

