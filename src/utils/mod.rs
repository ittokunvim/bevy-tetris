use bevy::prelude::*;

pub mod block;
pub mod blockdata;
pub mod prelude;
pub mod wall;

pub struct UtilsPlugin;

impl Plugin for UtilsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(block::BlockPlugin)
        ;
    }
}
