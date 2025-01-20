use bevy::prelude::*;

use crate::{
    GRID_SIZE,
    AppState,
};
use super::BlockSpawnEvent;
use crate::utils::prelude::*;

const BLOCK_COUNT: usize = 4;

fn setup(
    mut events: EventWriter<BlockSpawnEvent>,
) {
    events.send_default();
}

fn spawn(
    mut events: EventReader<BlockSpawnEvent>,
    mut commands: Commands,
    current_block: Res<CurrentBlock>,
) {
    if events.is_empty() { return }
    events.clear();

    let blockdata_id = current_block.id;
    let block = current_block.block;
    // debug!("spawn");
    for block_id in 1..BLOCK_COUNT + 1 {
        commands.spawn(Block::new(blockdata_id, block_id, block));
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

pub struct BlockPlugin;

impl Plugin for BlockPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::Ingame), setup)
            .add_systems(Update, (
                spawn,
                check_position,
            ).run_if(in_state(AppState::Ingame)))
            .add_systems(OnExit(AppState::Ingame), despawn_all)
        ;
    }
}
