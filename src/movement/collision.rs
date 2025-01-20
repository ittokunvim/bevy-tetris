use bevy::prelude::*;

use crate::GRID_SIZE;

use super::{
    MoveEvent,
    Direction,
    WallCollisionEvent,
    BlockCollisionEvent,
    BottomHitEvent,
    WallTopHitEvent,
};
use crate::utils::prelude::*;

pub fn check_for_wall(
    mut read_events: EventReader<MoveEvent>,
    mut write_events1: EventWriter<WallCollisionEvent>,
    mut write_events2: EventWriter<BottomHitEvent>,
    mut write_events3: EventWriter<WallTopHitEvent>,
    player_query: Query<&Transform, (With<PlayerBlock>, Without<Wall>)>,
    wall_query: Query<(&Wall, &Transform), (With<Wall>, Without<PlayerBlock>)>,
) {
    for block_move_event in read_events.read() {
        let direction = block_move_event.0;
        // send event closure
        let mut closure = |location: WallLocation| {
            // trace!("location: {:?}", location);
            write_events1.send_default();
            if location == WallLocation::Bottom { write_events2.send_default(); }
            if location == WallLocation::Top    { write_events3.send_default(); }
        };
        // check collision wall
        for player_transform in &player_query {
            let (mut player_x, mut player_y) = (
                player_transform.translation.x,
                player_transform.translation.y,
            );
            match direction {
                Direction::Left   => player_x -= GRID_SIZE,
                Direction::Right  => player_x += GRID_SIZE,
                Direction::Bottom => player_y -= GRID_SIZE,
            }
            for (wall, wall_transform) in &wall_query {
                let (wall_x, wall_y) = (
                    wall_transform.translation.x,
                    wall_transform.translation.y,
                );
                match wall.location {
                    WallLocation::Left =>
                    if player_x <= wall_x { closure(wall.location) }
                    WallLocation::Right =>
                    if player_x >= wall_x { closure(wall.location) }
                    WallLocation::Bottom =>
                    if player_y <= wall_y { closure(wall.location) }
                    WallLocation::Top =>
                    if player_y >= wall_y { closure(wall.location) }
                }
            }
        }
    }
}

pub fn check_for_block(
    mut read_events: EventReader<MoveEvent>,
    mut write_events1: EventWriter<BlockCollisionEvent>,
    mut write_events2: EventWriter<BottomHitEvent>,
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
