use bevy::prelude::*;

mod block;

#[derive(Event, Default)]
pub struct BlockSpawnEvent;

pub struct SpawnPlugin;

impl Plugin for SpawnPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<BlockSpawnEvent>()
            .add_plugins(block::BlockPlugin)
        ;
    }
}
