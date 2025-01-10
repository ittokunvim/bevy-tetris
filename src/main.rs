use bevy::{
    prelude::*,
    log::LogPlugin,
};

mod utils;
mod blocks;
mod wall;

const GAMETITLE: &str = "テトリス";
const GRID_SIZE: f32 = 20.0;
const WINDOW_SIZE: Vec2 = Vec2::new(32.0, 24.0);
const BACKGROUND_COLOR: Color = Color::srgb(0.1, 0.1, 0.1);
const PATH_SOUND_BGM: &str = "bevy-tetris/bgm.ogg";

fn main() {
    App::new()
        .add_plugins(DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    resolution: (WINDOW_SIZE * GRID_SIZE).into(),
                    title: GAMETITLE.to_string(),
                    ..Default::default()
                }),
                ..Default::default()
            })
            .set(ImagePlugin::default_nearest())
            .set(LogPlugin {
                filter: "info,wgpu_core=warn,wgpu_hal=warn,bevy_tetris=trace".into(),
                level: bevy::log::Level::DEBUG,
                ..Default::default()
            })
        )
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .insert_resource(Time::<Fixed>::from_seconds(1.0 / 60.0))
        .add_systems(Startup, (
            setup_camera,
            setup_bgm,
        ))
        .add_plugins(blocks::BlocksPlugin)
        .add_plugins(wall::WallPlugin)
        .run();
}

fn setup_camera(
    mut commands: Commands,
) {
    // debug!("setup camera");
    commands.spawn(Camera2d::default());
}

fn setup_bgm(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let sound = AudioPlayer::new(asset_server.load(PATH_SOUND_BGM));
    let settings = PlaybackSettings::LOOP;
    // debug!("setup bgm");
    commands.spawn((sound, settings));
}
