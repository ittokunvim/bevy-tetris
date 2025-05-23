use bevy::{
    prelude::*,
    time::Stopwatch,
};
use crate::GRID_SIZE;
use crate::ingame::block::BlockType;

mod block;
mod blockdata;
mod field;
mod key;
mod nextblock;
mod holdblock;
mod utils;
mod scoreboard;

const BLOCK_FALL_SPEED: f32 = 0.5;
const BLOCK_MOVE_SPEED: f32 = 0.25;
const FIELD_SIZE: Vec2 = Vec2::new(10.0 * GRID_SIZE, 20.0 * GRID_SIZE);
const FIELD_POSITION: Vec3 = Vec3::new(0.0, 0.0, -10.0);

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

#[derive(Resource, Deref, DerefMut)]
struct FallingTimer(Timer);

impl FallingTimer {
    fn new() -> Self {
        Self(Timer::from_seconds(BLOCK_FALL_SPEED, TimerMode::Repeating))
    }
}

#[derive(Resource, Deref, DerefMut)]
struct MoveLeftTimer(Stopwatch);

#[derive(Resource, Deref, DerefMut)]
struct MoveRightTimer(Stopwatch);

#[derive(Resource, Deref, DerefMut)]
struct MoveBottomTimer(Stopwatch);

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
            .insert_resource(FallingTimer::new())
            .insert_resource(MoveLeftTimer(Stopwatch::new()))
            .insert_resource(MoveRightTimer(Stopwatch::new()))
            .insert_resource(MoveBottomTimer(Stopwatch::new()))
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
