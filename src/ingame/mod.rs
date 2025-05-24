use bevy::prelude::*;
use crate::ingame::utils::prelude::*;

mod block;
mod field;
mod key;
mod nextblock;
mod holdblock;
mod utils;
mod scoreboard;

#[derive(Event)]
struct MoveEvent(Direction);

#[derive(Event)]
struct RotationEvent(Direction);

#[derive(Event, Default)]
struct HardDropEvent;

#[derive(Event, Default)]
struct SpawnEvent;

#[derive(Event, Default)]
struct FixEvent;

#[derive(Event)]
struct HoldEvent(BlockType);

#[derive(Copy, Clone, PartialEq, Debug)]
enum Direction {
    Left,
    Right,
    Bottom,
}

pub struct IngamePlugin;

impl Plugin for IngamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<MoveEvent>()
            .add_event::<RotationEvent>()
            .add_event::<HardDropEvent>()
            .add_event::<SpawnEvent>()
            .add_event::<FixEvent>()
            .add_event::<HoldEvent>()
            .add_plugins(field::FieldPlugin)
            .add_plugins(key::KeyPlugin)
            .add_plugins(block::BlockPlugin)
            .add_plugins(nextblock::NextBlockPlugin)
            .add_plugins(holdblock::HoldBlockPlugin)
            .add_plugins(utils::UtilsPlugin)
            .add_plugins(scoreboard::ScoreboardPlugin)
        ;
    }
}
