use bevy::prelude::*;

use crate::{
    GRID_SIZE_HALF,
    PATH_FONT,
    AppState,
};
use super::{
    FIELD_SIZE,
    FIELD_POSITION,
    HoldEvent,
};
use super::utils::prelude::*;

const BOARD_SIZE: Vec2 = Vec2::new(
    GRID_SIZE_HALF * 6.0,
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

/// ホールドされたブロックを記憶するコンポーネント
#[derive(Component, Debug)]
struct HoldBlock {
    blocktype: Option<BlockType>,
    block_id: usize,
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
            HoldBlock { blocktype: None, block_id, }
        ));
    }
}

/// ホールドしたブロックを更新する関数
/// ブロックの状態をゲーム進行に合わせて更新を行う
fn update(
    mut commands: Commands,
    mut hold_events: EventReader<HoldEvent>,
    mut holdblock_query: Query<(
        &mut Transform,
        &mut MeshMaterial2d<ColorMaterial>,
        &mut HoldBlock
    ), With<HoldBlock>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    player_query: Query<Entity, With<PlayerBlock>>,
) {
    info_once!("update");

    // ホールドイベントが発生した時の処理
    for event in hold_events.read() {
        let blocktype = event.0;

        // プレイヤーブロックを削除する
        for entity in player_query.iter() {
            commands.entity(entity).despawn();
        }

        // ホールドされたブロックを表示を更新
        let blockdata = &blocktype.blockdata()[0];

        for (mut transform, mut color, mut holdblock) in &mut holdblock_query.iter_mut() {
            // 現在のホールドブロックIDが形状データに含まれるか検索
            if let Some((index, _)) = blockdata
                .iter()
                .enumerate()
                .find(|(_, &v)| v == holdblock.block_id)
            {
                // ブロックの色を更新
                *color = MeshMaterial2d(materials.add(blocktype.color()));
                // ブロックタイプを更新
                holdblock.blocktype = Some(blocktype);

                // ブロック表示位置を計算（タイプごとに微調整）
                let pos = blocktype.calculate_position(BLOCK_INIT_POSITION);
                // ブロックの座標を設定
                transform.translation = Vec3::new(
                    pos.x + GRID_SIZE_HALF * ((index % 4) as f32),
                    pos.y - GRID_SIZE_HALF * ((index / 4) as f32),
                    10.0,
                );
            }
        }
    }
}

/// ホールドブロックを削除する関数
/// ゲームオーバーから抜けた時に実行される
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
