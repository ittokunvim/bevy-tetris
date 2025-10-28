use bevy::prelude::*;

use crate::{
    GRID_SIZE_HALF,
    PATH_FONT,
    AppState,
};
use super::{
    FIELD_SIZE,
    FIELD_POSITION,
    BlockSpawned,
};
use super::utils::prelude::*;

const BOARD_SIZE: Vec2 = Vec2::new(
    GRID_SIZE_HALF * 6.0,
    GRID_SIZE_HALF * 20.0,
);
const BOARD_POSITION: Vec3 = Vec3::new(
    FIELD_POSITION.x + FIELD_SIZE.x / 2.0 + BOARD_SIZE.x / 2.0,
    FIELD_POSITION.y + FIELD_SIZE.y / 2.0 - BOARD_SIZE.y / 2.0,
    0.0,
);
const BOARD_COLOR: Color = Color::srgb(0.16, 0.18, 0.26);

const NEXT_TEXT: &str = "NEXT";
const NEXT_FONT_SIZE: f32 = 20.0;
const NEXT_POSITION: Vec3 = Vec3::new(
    BOARD_POSITION.x, 
    BOARD_POSITION.y + BOARD_SIZE.y / 2.0 - NEXT_FONT_SIZE / 2.0 - GRID_SIZE_HALF * 1.75,
    10.0, 
);

const BLOCK_SIZE: Vec2 = Vec2::new(GRID_SIZE_HALF, GRID_SIZE_HALF);
const BLOCK_INIT_POSITION: Vec3 = Vec3::new(
    BOARD_POSITION.x - BOARD_SIZE.x / 2.0 + BLOCK_SIZE.x / 2.0,
    BOARD_POSITION.y + BOARD_SIZE.y / 2.0 - BLOCK_SIZE.y / 2.0 - GRID_SIZE_HALF * 5.0,
    10.0,
);

#[derive(Component)]
pub struct NextBoard;

/// 次にくるブロックを記憶するコンポーネント
#[derive(Component, Debug)]
struct NextBlock {
    nextblock_id: usize,
    blocktype: BlockType,
    block_id: usize,
}

/// 次にくるブロックを描画する関数
/// フィールド右上に配置し、次回に生成される
/// ブロックの形を3つ表示する
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
        NextBoard,
    ));

    // テキストを生成する
    let font = asset_server.load(PATH_FONT);
    commands.spawn((
        Text2d::new(NEXT_TEXT),
        TextFont {
            font,
            font_size: NEXT_FONT_SIZE,
            ..Default::default()
        },
        Transform::from_translation(NEXT_POSITION),
        NextBoard,
    ));

    // 空の次ブロックを生成する
    let shape = meshes.add(Rectangle::new(BLOCK_SIZE.x, BLOCK_SIZE.y));
    let color = Color::NONE;
    let blocktype = BlockType::TypeI;
    for nextblock_id in 1..=NEXT_BLOCK_COUNT - 1 {
        for block_id in 1..=BLOCK_UNIT_COUNT {
            commands.spawn((
                Mesh2d(shape.clone()),
                MeshMaterial2d(materials.add(color)),
                NextBoard,
                NextBlock { nextblock_id, blocktype, block_id, },
            ));
        }
    }
}

/// ブロック生成時に次にくるブロックの更新を行う関数
/// 次ブロックリストの値の更新し画面の更新も行う
fn update(
    _spawned: On<BlockSpawned>,
    mut query: Query<(
        &mut Transform,
        &mut MeshMaterial2d<ColorMaterial>,
        &mut NextBlock
    ), With<NextBlock>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    nextblocks: Res<NextBlocks>,
) {
    info_once!("update");

    // 次ブロック一覧をループ
    for (mut transform, mut color, mut nextblock) in &mut query {
        let nextblock_id = nextblock.nextblock_id;
        let block_id = nextblock.block_id;
        let blocktype = nextblocks[nextblock_id];

        // ブロックの色を更新
        *color = MeshMaterial2d(materials.add(blocktype.color()));
        // ブロックの形を更新
        nextblock.blocktype = blocktype;

        // 現在のブロックデータ配列内に該当するindexを検索
        if let Some((index, _)) = blocktype.blockdata()[0]
            .iter()
            .enumerate()
            .find(|(_, &blockdata_value)| blockdata_value == block_id)
        {
            // ブロックの描画y座標を計算
            let y = BLOCK_INIT_POSITION.y - GRID_SIZE_HALF * 5.0 * (nextblock_id - 1) as f32;
            // 初期位置y座標反映＋ブロックタイプごとのオフセット計算
            let init_position = blocktype.calculate_position(BLOCK_INIT_POSITION.with_y(y));

            // インデックスからブロックの座標を計算し、位置を更新
            transform.translation = Vec3::new(
                init_position.x + GRID_SIZE_HALF * ((index % 4) as f32),
                init_position.y - GRID_SIZE_HALF * ((index / 4) as f32),
                10.0,
            );
        }
    }
}

/// 次にくるブロックを削除する関数
fn despawn(
    mut commands: Commands,
    query: Query<Entity, With<NextBoard>>,
) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

pub struct NextBlockPlugin;

impl Plugin for NextBlockPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::InGame), setup)
            .add_observer(update)
            .add_systems(OnExit(AppState::Gameover), despawn)
        ;
    }
}
