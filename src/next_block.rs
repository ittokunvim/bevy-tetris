use bevy::prelude::*;

use crate::{
    GRID_SIZE_HALF,
    PATH_FONT,
    FIELD_SIZE,
    FIELD_POSITION,
    SpawnEvent,
    AppState,
};
use crate::block::{
    BlockType,
    NextBlocks,
};

const BOARD_SIZE: Vec2 = Vec2::new(
    GRID_SIZE_HALF * 5.0,
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
    BOARD_POSITION.y + GRID_SIZE_HALF * 9.0 - NEXT_FONT_SIZE / 2.0, 
    10.0, 
);

const BLOCK_SIZE: Vec2 = Vec2::new(GRID_SIZE_HALF, GRID_SIZE_HALF);
const BLOCK_INIT_POSITION: Vec3 = Vec3::new(
    BOARD_POSITION.x - BOARD_SIZE.x / 2.0 + BLOCK_SIZE.x / 2.0,
    BOARD_POSITION.y + BOARD_SIZE.y / 2.0 - BLOCK_SIZE.y / 2.0 - GRID_SIZE_HALF * 5.0,
    10.0,
);

#[derive(Component)]
pub struct NextBlockMenu;

#[derive(Component, Debug)]
struct NextBlockData {
    nextblock_id: usize,
    blocktype: BlockType,
    block_id: usize,
}

/// 次に出てくるブロックを描画する関数
/// フィールド右上に配置し、次回に生成される
/// ブロックの形を3つ表示する
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    next_block: Res<NextBlocks>,
    asset_server: Res<AssetServer>,
) {
    info_once!("setup");

    // ボードを生成する
    commands.spawn((
        Sprite::from_color(BOARD_COLOR, BOARD_SIZE),
        Transform::from_translation(BOARD_POSITION),
        NextBlockMenu,
    ));

    // テキストのフォントをロード
    let font = asset_server.load(PATH_FONT);

    // テキストを生成する
    commands.spawn((
        Text2d::new(NEXT_TEXT),
        TextFont {
            font,
            font_size: NEXT_FONT_SIZE,
            ..Default::default()
        },
        Transform::from_translation(NEXT_POSITION),
    ));

    // ブロックの大きさと四角形を定義
    let shape = meshes.add(Rectangle::new(BLOCK_SIZE.x, BLOCK_SIZE.y));

    // 次に生成されるブロックのリストをループ
    for (nextblock_id, blocktype) in next_block.0.iter().enumerate() {
        if nextblock_id <= 0 {
            continue;
        }

        let color = blocktype.color();
        let init_position = BLOCK_INIT_POSITION.with_y(GRID_SIZE_HALF * 5.0 * nextblock_id as f32);
        let init_position = calculate_nextblock_position(blocktype, init_position);

        // BlockTypeからBlockDataを取得しループ
        for (block_id, value) in blocktype.blockdata()[0].iter().enumerate() {
            // ブロックの値が0であればスキップ
            if *value == 0 {
                continue;
            }

            // ブロックの位置を計算
            let translation = Vec3::new(
                init_position.x + GRID_SIZE_HALF * ((block_id % 4) as f32),
                init_position.y - GRID_SIZE_HALF * ((block_id / 4) as f32),
                10.0,
            );

            // 1つのブロックを生成
            commands.spawn((
                Mesh2d(shape.clone()),
                MeshMaterial2d(materials.add(color)),
                Transform::from_translation(translation),
                NextBlockMenu,
                NextBlockData {
                    nextblock_id,
                    blocktype: *blocktype,
                    block_id: *value,
                },
            ));
        }
    }
}

/// ブロック生成時に次にくるブロックの更新を行う関数
/// 次ブロックリストの値の更新し画面の更新も行う
fn update(
    mut events: EventReader<SpawnEvent>,
    mut query: Query<(
        &mut Transform,
        &mut MeshMaterial2d<ColorMaterial>,
        &mut NextBlockData
    ), With<NextBlockData>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut nextblock: ResMut<NextBlocks>,
) {
    info_once!("update");

    // ブロック生成イベント時に処理を実行
    if events.is_empty() {
        return;
    }

    // イベントをクリア
    events.clear();

    // 次ブロックデータを更新
    *nextblock = nextblock.update();

    // 次ブロック一覧をループ
    for (mut transform, mut color, mut nextblockdata) in &mut query {
        // 初めの次ブロックは対象外
        if nextblockdata.nextblock_id <= 0 {
            continue;
        }

        let nextblock_id = nextblockdata.nextblock_id;
        let prev_blocktype = nextblockdata.blocktype;
        let block_id = nextblockdata.block_id;

        // 次ブロックの色を更新
        *color = MeshMaterial2d(materials.add(nextblockdata.blocktype.color()));
        // 次ブロックのブロックの形を更新
        nextblockdata.blocktype = nextblock.0[nextblock_id];

        // 現在のブロックデータ配列内に該当するindexを検索
        if let Some((index, _)) = prev_blocktype.blockdata()[0]
            .iter()
            .enumerate()
            .find(|(_, &blockdata_value)| blockdata_value == block_id)
        {
            // ブロックの描画y座標を計算
            let y = BLOCK_INIT_POSITION.y - GRID_SIZE_HALF * 5.0 * (nextblock_id - 1) as f32;
            // 初期位置y座標反映＋ブロックタイプごとのオフセット計算
            let init_position = calculate_nextblock_position(
                &prev_blocktype,
                BLOCK_INIT_POSITION.with_y(y),
            );
            // インデックスからブロックの座標を計算し、位置を更新
            transform.translation = Vec3::new(
                init_position.x + GRID_SIZE_HALF * ((index % 4) as f32),
                init_position.y - GRID_SIZE_HALF * ((index / 4) as f32),
                10.0,
            );
        }
    }
}

fn despawn(
    mut commands: Commands,
    query: Query<Entity, With<NextBlockMenu>>,
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
            .add_systems(Update, update.run_if(in_state(AppState::InGame)))
            .add_systems(OnExit(AppState::Gameover), despawn)
        ;
    }
}

/// 次ブロックの生成ポジションを
/// 各ブロックの種類に応じて微調整する関数
fn calculate_nextblock_position(
    blocktype: &BlockType,
    init_position: Vec3,
) -> Vec2 {
    match blocktype {
        BlockType::TypeI => Vec2::new(
            init_position.x + GRID_SIZE_HALF * 0.5,
            init_position.y - GRID_SIZE_HALF * 1.0,
        ),
        BlockType::TypeJ => Vec2::new(
            init_position.x + GRID_SIZE_HALF * 1.0,
            init_position.y - GRID_SIZE_HALF * 1.5,
        ),
        BlockType::TypeL => Vec2::new(
            init_position.x + GRID_SIZE_HALF * 1.0,
            init_position.y - GRID_SIZE_HALF * 1.5,
        ),
        BlockType::TypeO => Vec2::new(
            init_position.x + GRID_SIZE_HALF * 0.5,
            init_position.y - GRID_SIZE_HALF * 0.5,
        ),
        BlockType::TypeS => Vec2::new(
            init_position.x + GRID_SIZE_HALF * 1.0,
            init_position.y - GRID_SIZE_HALF * 0.5,
        ),
        BlockType::TypeT => Vec2::new(
            init_position.x + GRID_SIZE_HALF * 1.0,
            init_position.y - GRID_SIZE_HALF * 1.5,
        ),
        BlockType::TypeZ => Vec2::new(
            init_position.x + GRID_SIZE_HALF * 1.0,
            init_position.y - GRID_SIZE_HALF * 0.5,
        ),
    }
}
