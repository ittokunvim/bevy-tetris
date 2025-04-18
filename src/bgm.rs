use bevy::prelude::*;

use crate::{
    PATH_SOUND_BGM,
    AppState,
};

#[derive(Component)]
struct Bgm;

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let sound = AudioPlayer::new(asset_server.load(PATH_SOUND_BGM));
    let settings = PlaybackSettings::LOOP;

    commands.spawn((sound, settings, Bgm));
}

fn despawn(
    mut commands: Commands,
    query: Query<Entity, With<Bgm>>,
) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

pub struct BgmPlugin;

impl Plugin for BgmPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::InGame), setup)
            .add_systems(OnExit(AppState::InGame), despawn)
        ;
    }
}
