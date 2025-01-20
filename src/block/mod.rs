use bevy::prelude::*;

pub mod collision;
pub mod movement;
pub mod rotation;
mod spawn;

#[derive(Event, Default)]
pub struct RotationEvent;

#[derive(Event, Default)]
pub struct CollisionEvent;

pub struct BlockPlugin;

impl Plugin for BlockPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<RotationEvent>()
            .add_event::<CollisionEvent>()
            // .add_plugins(collision::CollisionPlugin)
            .add_plugins(movement::MovementPlugin)
            // .add_plugins(rotation::RotationPlugin)
            .add_plugins(spawn::SpawnPlugin)
        ;
    }
}
