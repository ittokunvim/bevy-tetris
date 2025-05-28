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
            .add_systems(Update, (
                spawn::block_spawn,
                movement::block_falling,
                rotation::block_rotation,
                movement::block_movement,
                harddrop::block_harddrop,
                hold::block_hold,
                gizmos::draw_gizmos_block,
                fix::clear_block,
                fix::enable_hold,
                fix::check_gameover,
            ).chain().run_if(in_state(AppState::InGame)))
        ;
    }
}
