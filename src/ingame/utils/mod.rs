use bevy::prelude::*;

pub mod prelude;

mod blockdata;
mod blockrandomizer;

pub struct UtilsPlugin;

impl Plugin for UtilsPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(blockrandomizer::BlockRandomizer::new())
        ;
    }
}
