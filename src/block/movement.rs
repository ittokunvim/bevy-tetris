use bevy::prelude::*;

use crate::GRID_SIZE;
use crate::player::{
    BlockDirection,
    BlockMoveEvent,
};
use super::PlayerBlock;

const FPS: f32 = 0.2;

#[derive(Resource, Deref, DerefMut)]
pub struct FallingTimer(Timer);

impl Default for FallingTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(FPS, TimerMode::Repeating))
    }
}

pub fn falling(
    mut timer: ResMut<FallingTimer>,
    mut events: EventWriter<BlockMoveEvent>,
    time: Res<Time>,
) {
    timer.tick(time.delta());
    if !timer.just_finished() { return }
    timer.reset();
    // debug!("falling block");
    events.send(BlockMoveEvent(BlockDirection::Bottom));
}

pub fn movement(
    mut events: EventReader<BlockMoveEvent>,
    mut query: Query<&mut Transform, With<PlayerBlock>>,
) {
    for event in events.read() {
        let direction = event.0;
        // trace!("direction: {:?}", direction);
        for mut transform in &mut query {
            match direction {
                BlockDirection::Left   => transform.translation.x -= GRID_SIZE,
                BlockDirection::Right  => transform.translation.x += GRID_SIZE,
                BlockDirection::Bottom => transform.translation.y -= GRID_SIZE,
            }
        }
    }
}

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(FallingTimer::default())
            // .add_systems(Update, falling)
            // .add_systems(Update, movement)
        ;
    }
}
