use bevy::prelude::*;

use crate::ingame::utils::prelude::*;

mod block;
mod field;
mod key;
mod nextblock;
mod holdblock;
mod utils;
mod scoreboard;

/// ブロック移動イベント（左右下移動）
#[derive(Event)]
struct MoveEvent(Direction);

/// ブロック回転イベント（左右回転）
#[derive(Event)]
struct RotationEvent(Direction);

/// ハードドロップイベント
#[derive(Event, Default)]
struct HardDropEvent;

/// ブロック生成イベント
/// 引数にブロックタイプが指定されたらそのブロックを生成
/// なければランダムに生成
#[derive(Event, Default)]
struct SpawnEvent(Option<BlockType>);

/// ブロック固定イベント
#[derive(Event, Default)]
struct FixEvent;

/// ブロックホールドイベント
#[derive(Event)]
struct HoldEvent(BlockType);

/// ブロックの移動や回転の方向
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
