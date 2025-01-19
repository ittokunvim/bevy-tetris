use bevy::prelude::*;

use crate::AppState;

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
struct BottomHitEvent;

#[derive(Event, Default)]
struct WallTopHitEvent;

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<MoveEvent>()
            .add_event::<BottomHitEvent>()
            .add_event::<WallTopHitEvent>()
            .add_systems(Update, (
                events::movement,
            ).chain().run_if(in_state(AppState::Ingame)))
            .add_systems(Update, (
                events::bottom_hit,
                events::top_hit,
            ).run_if(in_state(AppState::Ingame)))
        ;
    }
}
