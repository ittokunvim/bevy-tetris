use bevy::prelude::*;

use crate::player::{
    BlockMoveEvent,
    BlockDirection,
};
use crate::block::spawn::Block;
use super::{
    PlayerBlock,
};

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
                        BlockDirection::Left   => { debug!("collision left") }
                        BlockDirection::Right  => { debug!("collision right") }
                        BlockDirection::Bottom => { debug!("collision bottom") }
                    }
                    // trace!("send collision event");
                    write_events.send_default();
                    return;
                }
            }
        }
    }
}

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<CollisionEvent>()
            // .add_systems(Update, ( // move block/movement.rs
            //     // check_for_collision,
            // ))
        ;
    }
}
