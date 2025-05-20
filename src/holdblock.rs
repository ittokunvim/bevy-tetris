use bevy::prelude::*;

use crate::blockdata::BLOCK_UNIT_COUNT;
use crate::{
    GRID_SIZE_HALF,
    PATH_FONT,
    FIELD_SIZE,
    FIELD_POSITION,
    AppState,
};

use crate::block::{
    BlockType,
};

const BOARD_SIZE: Vec2 = Vec2::new(
    GRID_SIZE_HALF * 5.0,
    GRID_SIZE_HALF * 9.0,
);
const BOARD_POSITION: Vec3 = Vec3::new(
    FIELD_POSITION.x - FIELD_SIZE.x / 2.0 - BOARD_SIZE.x / 2.0,
    FIELD_POSITION.y + FIELD_SIZE.y / 2.0 - BOARD_SIZE.y / 2.0,
    0.0,
);
const BOARD_COLOR: Color = Color::srgb(0.16, 0.18, 0.26);

const HOLD_TEXT: &str = "HOLD";
const HOLD_FONT_SIZE: f32 = 20.0;
const HOLD_POSITION: Vec3 = Vec3::new(
    BOARD_POSITION.x,
    BOARD_POSITION.y + BOARD_SIZE.y / 2.0 - HOLD_FONT_SIZE / 2.0 - HOLD_PADDING,
    10.0, 
);
const HOLD_PADDING: f32 = GRID_SIZE_HALF * 1.75;

const BLOCK_SIZE: Vec2 = Vec2::new(GRID_SIZE_HALF, GRID_SIZE_HALF);
const BLOCK_INIT_POSITION: Vec3 = Vec3::new(
    BOARD_POSITION.x - BOARD_SIZE.x / 2.0 + BLOCK_SIZE.x / 2.0,
    BOARD_POSITION.y + BOARD_SIZE.y / 2.0 - BLOCK_SIZE.y / 2.0 - GRID_SIZE_HALF * 4.0,
    10.0,
);

#[derive(Component)]
struct HoldBoard;

#[derive(Component, Debug)]
struct HoldBlock {
}

/// ホールドされたブロックを描画する関数
/// フィールド左上に配置し、初めは空の状態で描画する
/// その後ホールドされたら、そのブロックを表示する
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    info_once!("setup");

    // ボードを生成する
    commands.spawn((
        Sprite::from_color(BOARD_COLOR, BOARD_SIZE),
        Transform::from_translation(BOARD_POSITION),
        HoldBoard,
    ));

    // テキストを生成する
    let font = asset_server.load(PATH_FONT);
    commands.spawn((
        Text2d::new(HOLD_TEXT),
        TextFont {
            font,
            font_size: HOLD_FONT_SIZE,
            ..Default::default()
        },
        Transform::from_translation(HOLD_POSITION),
        HoldBoard,
    ));

    // 空のブロックを生成する
    let shape = meshes.add(Rectangle::new(BLOCK_SIZE.x, BLOCK_SIZE.y));
    let color = Color::NONE;
    for block_id in 1..=BLOCK_UNIT_COUNT {
        commands.spawn((
            Mesh2d(shape.clone()),
            MeshMaterial2d(materials.add(color)),
            HoldBoard,
            HoldBlock { }
        ));
    }
}

fn update(
) {
    info_once!("update");
}

fn despawn(
    mut commands: Commands,
    query: Query<Entity, With<HoldBoard>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

pub struct HoldBlockPlugin;

impl Plugin for HoldBlockPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::InGame), setup)
            .add_systems(Update, update.run_if(in_state(AppState::InGame)))
            .add_systems(OnExit(AppState::Gameover), despawn)
        ;
    }
}

