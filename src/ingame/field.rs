use bevy::prelude::*;

use crate::AppState;
use super::{
    FIELD_SIZE,
    FIELD_POSITION,
};

const FIELD_COLOR: Color = Color::srgb(0.13, 0.14, 0.21);

#[derive(Component)]
struct Field;

/// フィールドのセットアップを行う関数
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    info_once!("setup");

    // フィールドを作成
    let shape = meshes.add(Rectangle::new(FIELD_SIZE.x, FIELD_SIZE.y));
    commands.spawn((
        Mesh2d(shape),
        MeshMaterial2d(materials.add(FIELD_COLOR)),
        Transform::from_xyz(FIELD_POSITION.x, FIELD_POSITION.y, FIELD_POSITION.z),
        Field,
    ));
}

/// フィールドを削除する関数
/// ステートがゲームオーバーから抜けた時に実行されます
fn despawn(
    mut commands: Commands,
    query: Query<Entity, With<Field>>,
) {
    info_once!("despawn");

    for entity in &query {
        commands.entity(entity).despawn();
    }
}

pub struct FieldPlugin;

impl Plugin for FieldPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::InGame), setup)
            .add_systems(OnExit(AppState::Gameover), despawn)
        ;
    }
}
