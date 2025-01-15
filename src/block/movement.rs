use bevy::prelude::*;

use crate::GRID_SIZE;
use super::PlayerBlock;
use super::collision::CollisionEvent as BlockCollisionEvent;
use crate::wall::CollisionEvent as WallCollisionEvent;

const FPS: f32 = 0.2;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Direction {
    Left,
    Right,
    Bottom,
}

#[derive(Event)]
pub struct MoveEvent(pub Direction);

#[derive(Resource, Deref, DerefMut)]
pub struct FallingTimer(Timer);

impl Default for FallingTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(FPS, TimerMode::Repeating))
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
    }
}

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<MoveEvent>()
            .insert_resource(FallingTimer::default())
            // .add_systems(Update, falling)
            // .add_systems(Update, movement)
        ;
    }
}
