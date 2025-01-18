use bevy::prelude::*;

use crate::{
    GRID_SIZE,
    AppState,
};
use crate::utils::blockdata::*;
use super::{
    BLOCK_SIZE,
    BLOCK_POSITION,
    SpawnEvent,
    CurrentBlock,
    PlayerBlock,
    Block,
    BlockType,
};

impl BlockType {
    fn color(&self) -> Color {
        match self {
            BlockType::TypeI => I_COLOR,
            BlockType::TypeJ => J_COLOR,
            BlockType::TypeL => L_COLOR,
            BlockType::TypeO => O_COLOR,
            BlockType::TypeS => S_COLOR,
            BlockType::TypeT => T_COLOR,
            BlockType::TypeZ => Z_COLOR,
        }
    }
    // このメソッドではブロックを生成する時のポジション(x,y)を返します。
    // ブロックデータは配列x4のusizex16からなり、usizex16から縦4マス横4マスの
    // ブロックの範囲とします。
    // そこからメソッドに渡されたIDを元にブロックデータの値を参照し、
    // 一致した箇所をxy軸に変換して返します。
    // 例
    // BlockType::TypeI.position(2) -> INITIAL_POSITION + Vec2::new(GRID_SIZE * 1, -GRID_SIZE * 1)
    fn position(&self, blockdata_id: usize, block_id: usize) -> Vec2 {
        let closure = |block: [[usize; 16]; 4]| {
            let mut position = BLOCK_POSITION;

            for i in 0..block[blockdata_id].len() {
                if block_id == block[blockdata_id][i] {
                    // trace!("position: {}", position);
                    return position
                }
                position.x += GRID_SIZE;
                // movement y
                if i % 4 == 3 {
                    position.x = BLOCK_POSITION.x;
                    position.y -= GRID_SIZE;
                }
            }
            // id should be in the block[0]
            panic!("id: {} is not found", block_id);
        };

        match self {
            BlockType::TypeI => closure(I_BLOCK),
            BlockType::TypeJ => closure(J_BLOCK),
            BlockType::TypeL => closure(L_BLOCK),
            BlockType::TypeO => closure(O_BLOCK),
            BlockType::TypeS => closure(S_BLOCK),
            BlockType::TypeT => closure(T_BLOCK),
            BlockType::TypeZ => closure(Z_BLOCK),
        }
    }
}

impl Block {
    fn new(
        blockdata_id: usize,
        block_id: usize,
        block: BlockType,
    ) -> (Self, Sprite, Transform) {
        (
            Self,
            Sprite::from_color(block.color(), Vec2::ONE),
            Transform {
                translation: block.position(blockdata_id, block_id).extend(1.0),
                scale: BLOCK_SIZE.extend(1.0),
                ..Default::default()
            },
        )
    }
}

fn setup(
    mut events: EventWriter<SpawnEvent>,
) {
    events.send_default();
}

fn spawn(
    mut events: EventReader<SpawnEvent>,
    mut commands: Commands,
    current_block: Res<CurrentBlock>,
) {
    if events.is_empty() { return }
    events.clear();

    let blockdata_id = current_block.id;
    let block = current_block.block;
    // debug!("spawn");
    for block_id in 1..BLOCK_COUNT + 1 {
        // commands.spawn(Block::new(i, BlockType::TypeI));
        // commands.spawn(Block::new(i, BlockType::TypeJ));
        // commands.spawn(Block::new(i, BlockType::TypeL));
        commands.spawn(Block::new(blockdata_id, block_id, block));
        // commands.spawn(Block::new(i, BlockType::TypeS));
        // commands.spawn(Block::new(i, BlockType::TypeT));
        // commands.spawn(Block::new(i, BlockType::TypeZ));
    }
}

fn check_position(
    mut player_query: Query<&mut Transform, With<PlayerBlock>>,
    mut current_block: ResMut<CurrentBlock>,
    block_query: Query<&Transform, (With<Block>, Without<PlayerBlock>)>,
) {
    let mut collision = true;
    // check PlayerBlock position
    while collision {
        collision = false;
        for player_transform in &player_query {
            let player_pos = player_transform.translation.truncate();
            for block_transform in &block_query {
                let block_pos = block_transform.translation.truncate();
                if player_pos == block_pos {
                    collision = true;
                    break;
                }
            }
        }
        if collision {
            // move PlayerBlock position
            for mut transform in &mut player_query {
                transform.translation.y += GRID_SIZE;
            }
            current_block.init_pos.y += GRID_SIZE;
        }
    }
}

fn despawn_all(
    mut commands: Commands,
    query: Query<Entity, With<Block>>,
) {
    // debug!("despawn_all");
    for entity in &query { commands.entity(entity).despawn() }
}

pub struct SpawnPlugin;

impl Plugin for SpawnPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<SpawnEvent>()
            .add_systems(OnEnter(AppState::Ingame), setup)
            .add_systems(Update, (
                spawn,
                check_position,
            ).run_if(in_state(AppState::Ingame)))
            .add_systems(OnExit(AppState::Ingame), despawn_all)
        ;
    }
}
