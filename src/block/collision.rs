use bevy::prelude::*;

use crate::GRID_SIZE;
use super::PlayerBlock;
use super::movement::{
    Direction,
    MoveEvent,
};
use super::spawn::Block;
use crate::wall::BottomHitEvent as BlockBottomHitEvent;

#[derive(Event, Default)]
pub struct CollisionEvent;

pub fn check_for_collision(
    mut read_events: EventReader<MoveEvent>,
    mut write_events1: EventWriter<CollisionEvent>,
    mut write_events2: EventWriter<BlockBottomHitEvent>,
    player_query: Query<&Transform, With<PlayerBlock>>,
    block_query: Query<&Transform, (With<Block>, Without<PlayerBlock>)>,
) {
    for event in read_events.read() {
        let direction = event.0;
        // debug!("check_for_collision");
        for player_transform in &player_query {
            let mut player_pos = player_transform.translation;
            match direction {
                Direction::Left   => player_pos.x -= GRID_SIZE,
                Direction::Right  => player_pos.x += GRID_SIZE,
                Direction::Bottom => player_pos.y -= GRID_SIZE,
            }
            // trace!("player_pos: {}", player_pos);
            for block_transform in &block_query {
                let block_pos = block_transform.translation;
                // trace!("block_pos: {}", block_pos);
                if player_pos == block_pos {
                    write_events1.send_default();
                    if direction == Direction::Bottom {
                        write_events2.send_default();
                    }
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
            // .add_systems(Update, check_for_collision)
        ;
    }
}
