use bevy::prelude::*;

use crate::{
    PATH_SOUND_BGM,
    PATH_SOUND_CLICK,
    AppState,
};

/// BGM用コンポーネント
#[derive(Component)]
struct Bgm;

/// クリックオンのハンドルを格納するリソース
#[derive(Resource, Deref)]
struct ClickSound(Handle<AudioSource>);

/// BGMを再生する関数
fn play_bgm(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    info_once!("play_bgm");

    let bgm = asset_server.load(PATH_SOUND_BGM);
    let sound = AudioPlayer::new(bgm);
    let settings = PlaybackSettings::LOOP;
    commands.spawn((sound, settings, Bgm));
}

/// BGMを止める関数
fn stop_bgm(
    mut commands: Commands,
    query: Query<Entity, With<Bgm>>,
) {
    info_once!("stop_bgm");

    for entity in &query {
        commands.entity(entity).despawn();
    }
}

/// クリック音のセットアップを行う関数
fn setup_click_sound(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    info_once!("setup_click_sound");

    let sound = asset_server.load(PATH_SOUND_CLICK);
    commands.insert_resource(ClickSound(sound));
}

/// クリック音を再生する関数
fn play_click_sound(
    mut commands: Commands,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    sound: Res<ClickSound>,
) {
    info_once!("play_click_sound");

    if mouse_buttons.just_pressed(MouseButton::Left) {
        let sound = AudioPlayer::new(sound.clone());
        let settings = PlaybackSettings::DESPAWN;
        commands.spawn((sound, settings));
    }
}

pub struct SoundPlugin;

impl Plugin for SoundPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::InGame), play_bgm)
            .add_systems(OnExit(AppState::InGame), stop_bgm)
            .add_systems(Startup, setup_click_sound)
            .add_systems(Update, play_click_sound.run_if(in_state(AppState::Mainmenu)))
            .add_systems(Update, play_click_sound.run_if(in_state(AppState::Gameover)))
        ;
    }
}
