use bevy::prelude::*;

use crate::AppState;

mod block;
mod events;

#[derive(Event)]
struct MoveEvent(pub Direction);

#[derive(Copy, Clone, PartialEq, Debug)]
enum Direction {
    Left,
    Right,
    Bottom,
}

#[derive(Event, Default)]
struct WallCollisionEvent;

#[derive(Event, Default)]
struct BlockCollisionEvent;

#[derive(Event, Default)]
struct BottomHitEvent;

#[derive(Event, Default)]
struct WallTopHitEvent;

#[derive(Resource, Deref, DerefMut)]
struct FallingTimer(Timer);

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<MoveEvent>()
            .add_event::<WallCollisionEvent>()
            .add_event::<BlockCollisionEvent>()
            .add_event::<BottomHitEvent>()
            .add_event::<WallTopHitEvent>()
            .insert_resource(FallingTimer::default())
            .add_systems(Update, (
                events::movement,
                block::falling,
                block::movement,
            ).chain().run_if(in_state(AppState::Ingame)))
            .add_systems(Update, (
                events::bottom_hit,
                events::top_hit,
            ).run_if(in_state(AppState::Ingame)))
            .add_systems(OnExit(AppState::Ingame), block::reset_timer)
        ;
    }
}
