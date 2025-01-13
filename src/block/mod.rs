use bevy::prelude::*;

use crate::GRID_SIZE;
use crate::player::{
    BlockDirection,
    BlockMoveEvent,
};

mod spawn;

const FPS: f32 = 0.2;

#[derive(Event, Default)]
struct SpawnEvent;

#[derive(Event, Default)]
pub struct ReachBottomEvent;

#[derive(Component, Deref, DerefMut)]
struct FallingTimer(Timer);

#[derive(Component, Default)]
pub struct PlayerBlock;

impl Default for FallingTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(FPS, TimerMode::Repeating))
    }
}

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

fn reach_bottom(
    mut read_events: EventReader<ReachBottomEvent>,
    mut write_events: EventWriter<SpawnEvent>,
    mut commands: Commands,
    query: Query<Entity, With<PlayerBlock>>,
) {
    if read_events.is_empty() { return }
    read_events.clear();
    // debug!("reach_bottom: remove PlayerBlock components");
    for entity in &query { commands.entity(entity).remove::<PlayerBlock>(); }
    // debug!("reach_bottom: send spawn event");
    write_events.send_default();
}

pub struct BlockPlugin;

impl Plugin for BlockPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<SpawnEvent>()
            .add_event::<ReachBottomEvent>()
            .add_plugins(spawn::SpawnPlugin)
            .add_systems(Update, (
                movement,
                falling,
                crate::wall::check_for_wall,
                reach_bottom,
            ).chain())
        ;
    }
}
