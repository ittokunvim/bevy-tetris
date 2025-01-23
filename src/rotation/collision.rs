use bevy::prelude::*;

use crate::GRID_SIZE;
use crate::utils::prelude::*;
use super::{
    BlockRotationEvent,
    BlockDirection,
};

pub fn check_for_wall(
    mut events: EventReader<BlockRotationEvent>,
    mut player_query: Query<&mut Transform, (With<PlayerBlock>, Without<Wall>)>,
    mut current_block: ResMut<CurrentBlock>,
    wall_query: Query<(&Wall, &Transform), (With<Wall>, Without<PlayerBlock>)>,
) {
    if events.is_empty() { return }
    events.clear();

    let mut collision = true;
    let mut direction = BlockDirection::Left;
    // check PlayerBlock position
    while collision {
        collision = false;
        for player_transform in &player_query {
            let player_pos = player_transform.translation.truncate();
            for (wall, wall_transform) in &wall_query {
                let wall_pos = wall_transform.translation.truncate();
                match wall.location {
                    WallLocation::Left => if player_pos.x <= wall_pos.x {
                        collision = true;
                        direction = BlockDirection::Left;
                        break;
                    }
                    WallLocation::Right => if player_pos.x >= wall_pos.x {
                        collision = true;
                        direction = BlockDirection::Right;
                        break;
                    }
                    WallLocation::Bottom => if player_pos.y <= wall_pos.y {
                        collision = true;
                        direction = BlockDirection::Bottom;
                        break;
                    }
                    WallLocation::Top => if player_pos.y >= wall_pos.y {
                        collision = true;
                        direction = BlockDirection::Top;
                        break;
                    }
                }
            }
        }
        if collision {
            // trace!("direction: {:?}", direction);
            for mut player_transform in &mut player_query {
                match direction {
                    BlockDirection::Left =>
                    player_transform.translation.x += GRID_SIZE,
                    BlockDirection::Right =>
                    player_transform.translation.x -= GRID_SIZE,
                    BlockDirection::Bottom =>
                    player_transform.translation.y += GRID_SIZE,
                    BlockDirection::Top =>
                    player_transform.translation.y -= GRID_SIZE,
                }
            }
            match direction {
                BlockDirection::Left   => current_block.init_pos.x += GRID_SIZE,
                BlockDirection::Right  => current_block.init_pos.x -= GRID_SIZE,
                BlockDirection::Bottom => current_block.init_pos.y += GRID_SIZE,
                BlockDirection::Top    => current_block.init_pos.y -= GRID_SIZE,
            }
        }
    }
}
