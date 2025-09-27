use bevy::prelude::*;

use crate::{
    WINDOW_SIZE,
    AppState,
};

const BUTTON_FIELD_SIZE: Vec2 = Vec2::new(WINDOW_SIZE.x, WINDOW_SIZE.y / 4.0);
const BUTTON_SIZE: f32 = 25.0;
const BUTTON_INIT_POSITION: Vec3 = Vec3::new(
    0.0,
    -WINDOW_SIZE.y / 2.0 + BUTTON_FIELD_SIZE.y / 2.0,
    -10.0
);
const BUTTON_COLOR: Color = Color::srgb(1.0, 1.0, 1.0);

#[derive(Component, Debug)]
struct KeyButton;

#[derive(Component, Debug)]
struct LeftButton;

#[derive(Component, Debug)]
struct RightButton;

#[derive(Component, Debug)]
struct TopButton;

#[derive(Component, Debug)]
struct BottomButton;

#[derive(Component, Debug)]
struct RotateLeftButton;

#[derive(Component, Debug)]
struct RotateRightButton;

#[derive(Component, Debug)]
struct HoldButton;

#[derive(Component, Debug)]
struct FallButton;

/// ゲームを操作するボタンのセットアップを行う関数
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    info_once!("setup");

    // ボタンフィールドを生成
    let shape = meshes.add(Rectangle::new(BUTTON_FIELD_SIZE.x, BUTTON_FIELD_SIZE.y));
    commands.spawn((
        Mesh2d(shape),
        MeshMaterial2d(materials.add(Color::BLACK)),
        Transform::from_translation(BUTTON_INIT_POSITION),
    ));

    // 8つのボタンを生成
    let shape = meshes.add(Circle::new(BUTTON_SIZE));
    for i in 0..8 {
        let mut pos = BUTTON_INIT_POSITION.with_z(-5.0);
        match i {
            // 左ボタンを生成
            0 => {
                pos.x = -BUTTON_FIELD_SIZE.x / 4.0 - BUTTON_SIZE * 2.0;
                commands.spawn((
                    Mesh2d(shape.clone()),
                    MeshMaterial2d(materials.add(BUTTON_COLOR)),
                    Transform::from_translation(pos),
                    KeyButton,
                    LeftButton,
                ));
            },
            // 右ボタンを生成
            1 => {
                pos.x = -BUTTON_FIELD_SIZE.x / 4.0 + BUTTON_SIZE * 2.0;
                commands.spawn((
                    Mesh2d(shape.clone()),
                    MeshMaterial2d(materials.add(BUTTON_COLOR)),
                    Transform::from_translation(pos),
                    KeyButton,
                    RightButton,
                ));
            },
            // 上ボタンを生成
            2 => {
                pos.x = -BUTTON_FIELD_SIZE.x / 4.0;
                pos.y -= BUTTON_SIZE * 2.0;
                commands.spawn((
                    Mesh2d(shape.clone()),
                    MeshMaterial2d(materials.add(BUTTON_COLOR)),
                    Transform::from_translation(pos),
                    KeyButton,
                    TopButton,
                ));
            },
            // 下ボタンを生成
            3 => {
                pos.x = -BUTTON_FIELD_SIZE.x / 4.0;
                pos.y += BUTTON_SIZE * 2.0;
                commands.spawn((
                    Mesh2d(shape.clone()),
                    MeshMaterial2d(materials.add(BUTTON_COLOR)),
                    Transform::from_translation(pos),
                    KeyButton,
                    BottomButton,
                ));
            },
            // 左回転ボタンを生成
            4 => {
                pos.x = BUTTON_FIELD_SIZE.x / 4.0 - BUTTON_SIZE * 2.0;
                commands.spawn((
                    Mesh2d(shape.clone()),
                    MeshMaterial2d(materials.add(BUTTON_COLOR)),
                    Transform::from_translation(pos),
                    KeyButton,
                    RotateLeftButton,
                ));
            },
            // 右回転ボタンを生成
            5 => {
                pos.x = BUTTON_FIELD_SIZE.x / 4.0 + BUTTON_SIZE * 2.0;
                commands.spawn((
                    Mesh2d(shape.clone()),
                    MeshMaterial2d(materials.add(BUTTON_COLOR)),
                    Transform::from_translation(pos),
                    KeyButton,
                    RotateRightButton,
                ));
            },
            // ホールドボタンを生成
            6 => {
                pos.x = BUTTON_FIELD_SIZE.x / 4.0;
                pos.y += BUTTON_SIZE * 2.0;
                commands.spawn((
                    Mesh2d(shape.clone()),
                    MeshMaterial2d(materials.add(BUTTON_COLOR)),
                    Transform::from_translation(pos),
                    KeyButton,
                    HoldButton,
                ));
            },
            // 落下ボタンを生成
            7 => {
                pos.x = BUTTON_FIELD_SIZE.x / 4.0;
                pos.y -= BUTTON_SIZE * 2.0;
                commands.spawn((
                    Mesh2d(shape.clone()),
                    MeshMaterial2d(materials.add(BUTTON_COLOR)),
                    Transform::from_translation(pos),
                    KeyButton,
                    FallButton,
                ));
            },
            _ => {},
        }
    }
}

pub struct ButtonPlugin;

impl Plugin for ButtonPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::InGame), setup)
        ;
    }
}
