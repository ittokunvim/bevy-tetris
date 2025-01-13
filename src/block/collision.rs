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

#[derive(Event)]
pub struct CollisionEvent(BlockDirection);

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
                    // debug!("send collision event");
                    write_events.send(CollisionEvent(direction));
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
    for event in read_events.read() {
        let direction = event.0;
        let mut closure = |direction: BlockDirection, movement: Vec3| {
            for (entity, mut transform) in &mut query {
                transform.translation += movement;
                // trace!("pos: {}", transform.translation);
                if direction == BlockDirection::Bottom {
                    commands.entity(entity).remove::<PlayerBlock>();
                    write_events.send_default();
                }
            }
        };
        let movement = Vec3::ZERO;
        // debug!("collision");
        match direction {
            BlockDirection::Left   => closure(direction, movement.with_x(GRID_SIZE)),
            BlockDirection::Right  => closure(direction, movement.with_x(-GRID_SIZE)),
            BlockDirection::Bottom => closure(direction, movement.with_y(GRID_SIZE)),
        }
    }
}

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<CollisionEvent>()
            // .add_systems(Update, check_for_collision)
            // .add_systems(Update, collision)
        ;
    }
}
