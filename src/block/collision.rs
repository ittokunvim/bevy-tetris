use bevy::prelude::*;

use crate::GRID_SIZE;
use crate::player::{
    BlockMoveEvent,
    BlockDirection,
};
use super::{
    SpawnEvent,
    PlayerBlock,
};
use super::spawn::Block;

#[derive(Event, Default)]
pub struct CollisionEvent;

pub fn check_for_collision(
    mut read_events: EventReader<BlockMoveEvent>,
    mut write_events: EventWriter<CollisionEvent>,
    player_query: Query<&Transform, With<PlayerBlock>>,
    block_query: Query<&Transform, (With<Block>, Without<PlayerBlock>)>,
) {
    for event in read_events.read() {
        let direction = event.0;
        // debug!("check_for_collision");
        for player_transform in &player_query {
            let player_pos = player_transform.translation;
            // trace!("player_pos: {}", player_pos);
            for block_transform in &block_query {
                let block_pos = block_transform.translation;
                // trace!("block_pos: {}", block_pos);
                if player_pos == block_pos {
                    match direction {
                        BlockDirection::Left   => {}
                        BlockDirection::Right  => {}
                        BlockDirection::Bottom => {}
                    }
                    // trace!("send collision event");
                    write_events.send_default();
                    return;
                }
            }
        }
    }
}

pub fn collision(
    mut read_events: EventReader<CollisionEvent>,
    mut write_events: EventWriter<SpawnEvent>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform), With<PlayerBlock>>,
) {
    if read_events.is_empty() { return }
    read_events.clear();
    // trace!("move block");
    for (entity, mut transform) in &mut query {
        transform.translation.y += GRID_SIZE;
        commands.entity(entity).remove::<PlayerBlock>();
    }
    write_events.send_default();
}

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<CollisionEvent>()
            // .add_systems(Update, ( // move block/movement.rs
            //     // check_for_collision,
            //     // collision,
            // ))
        ;
    }
}
