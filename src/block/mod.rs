use bevy::prelude::*;

pub mod collision;
pub mod movement;
pub mod spawn;

#[derive(Component, Default)]
pub struct PlayerBlock;

pub struct BlockPlugin;

impl Plugin for BlockPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(collision::CollisionPlugin)
            .add_plugins(movement::MovementPlugin)
            .add_plugins(spawn::SpawnPlugin)
        ;
    }
}
