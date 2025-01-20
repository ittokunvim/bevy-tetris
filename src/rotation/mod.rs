use bevy::prelude::*;

use crate::AppState;

mod events;

#[derive(Event, Default)]
pub struct BlockRotationEvent;

pub struct RotationPlugin;

impl Plugin for RotationPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<BlockRotationEvent>()
            .add_systems(Update, (
                events::rotation,
            ).chain().run_if(in_state(AppState::Ingame)))
        ;
    }
}
