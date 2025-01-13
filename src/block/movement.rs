use bevy::prelude::*;

use crate::GRID_SIZE;
use crate::player::{
    BlockDirection,
    BlockMoveEvent,
};
use super::{
    PlayerBlock,
    FallingTimer,
};

fn movement(
    mut events: EventReader<BlockMoveEvent>,
    mut query: Query<&mut Transform, With<PlayerBlock>>,
) {
    for event in events.read() {
        let direction = event.0;
        // trace!("direction: {:?}", direction);
        for mut transform in &mut query {
            match direction {
                BlockDirection::Left  => transform.translation.x -= GRID_SIZE,
                BlockDirection::Right => transform.translation.x += GRID_SIZE,
            }
        }
    }
}

fn falling(
    mut query: Query<(&mut FallingTimer, &mut Transform), With<PlayerBlock>>,
    time: Res<Time>,
) {
    for (mut timer, mut transform) in &mut query {
        timer.tick(time.delta());
        if !timer.just_finished() { continue }
        timer.reset();
        // debug!("movement");
        transform.translation.y -= GRID_SIZE;
    }
}

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (
                movement,
                falling,
                crate::wall::check_for_wall,
            ).chain())
        ;
    }
}
