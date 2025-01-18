use bevy::prelude::*;

use crate::{
    AppState,
    GRID_SIZE,
};
use super::{
    BLOCK_SPEED,
    CollisionEvent as BlockCollisionEvent,
    Direction,
    MoveEvent,
    CurrentBlock,
    PlayerBlock,
};
use crate::wall::CollisionEvent as WallCollisionEvent;

#[derive(Resource, Deref, DerefMut)]
pub struct FallingTimer(Timer);

impl Default for FallingTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(BLOCK_SPEED, TimerMode::Repeating))
    }
}

pub fn falling(
    mut timer: ResMut<FallingTimer>,
    mut events: EventWriter<MoveEvent>,
    time: Res<Time>,
) {
    timer.tick(time.delta());
    if !timer.just_finished() { return }
    timer.reset();
    // debug!("falling block");
    events.send(MoveEvent(Direction::Bottom));
}

pub fn movement(
    mut read_events1: EventReader<MoveEvent>,
    mut query: Query<&mut Transform, With<PlayerBlock>>,
    mut current_block: ResMut<CurrentBlock>,
    read_events2: EventReader<WallCollisionEvent>,
    read_events3: EventReader<BlockCollisionEvent>
) {
    if !read_events2.is_empty() { return }
    if !read_events3.is_empty()  { return }
    for event in read_events1.read() {
        let direction = event.0;
        // trace!("direction: {:?}", direction);
        for mut transform in &mut query {
            match direction {
                Direction::Left   => transform.translation.x -= GRID_SIZE,
                Direction::Right  => transform.translation.x += GRID_SIZE,
                Direction::Bottom => transform.translation.y -= GRID_SIZE,
            }
        }
        match direction {
            Direction::Left   => current_block.init_pos.x -= GRID_SIZE,
            Direction::Right  => current_block.init_pos.x += GRID_SIZE,
            Direction::Bottom => current_block.init_pos.y -= GRID_SIZE,
        }
        // trace!("current_block.init_pos: {}", current_block.init_pos);
    }
}

fn reset_timer(
    mut timer: ResMut<FallingTimer>,
) {
    // debug!("reset_timer");
    timer.reset();
}

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(FallingTimer::default())
            // .add_systems(Update, falling)
            // .add_systems(Update, movement)
            .add_systems(OnExit(AppState::Ingame), reset_timer)
        ;
    }
}
