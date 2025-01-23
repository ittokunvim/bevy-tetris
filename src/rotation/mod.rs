use bevy::prelude::*;

use crate::AppState;

mod block;
mod collision;
mod events;

#[derive(Event, Default)]
pub struct BlockRotationEvent;

#[derive(Debug)]
enum BlockDirection {
    Left,
    Right,
    Bottom,
    Top,
}

pub struct RotationPlugin;

impl Plugin for RotationPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<BlockRotationEvent>()
            .add_systems(Update, (
                events::rotation,
                block::rotation,
                collision::check_for_wall,
            ).chain().run_if(in_state(AppState::Ingame)))
        ;
    }
}
