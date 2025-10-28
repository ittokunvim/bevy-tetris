use bevy::prelude::*;

use crate::AppState;

mod fix;
mod gizmos;
mod harddrop;
mod hold;
mod movement;
mod rotation;
mod spawn;

pub struct BlockPlugin;

impl Plugin for BlockPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_observer(spawn::block_spawn)
            .add_observer(rotation::block_rotation)
            .add_observer(movement::block_movement)
            .add_observer(harddrop::block_harddrop)
            .add_observer(hold::block_hold)
            .add_observer(fix::clear_block)
            .add_observer(fix::enable_hold)
            .add_observer(fix::check_gameover)
            .add_systems(Update, (
                movement::block_falling,
                gizmos::draw_gizmos_block,
            ).chain().run_if(in_state(AppState::InGame)))
        ;
    }
}
