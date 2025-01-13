use bevy::prelude::*;

mod movement;
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
            .add_plugins(movement::MovementPlugin)
            .add_plugins(spawn::SpawnPlugin)
            .add_systems(Update, (
                reach_bottom,
            ).chain())
        ;
    }
}
