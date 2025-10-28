use bevy::prelude::*;

use crate::ingame::utils::prelude::*;

mod block;
mod buttons;
mod field;
mod nextblock;
mod holdblock;
mod utils;
mod scoreboard;

/// ブロック移動イベント（左右下移動）
#[derive(Event)]
struct BlockMoved(Direction);

/// ブロック回転イベント（左右回転）
#[derive(Event)]
struct BlockRotated(Direction);

/// ハードドロップイベント
#[derive(Event, Default)]
struct BlockHarddrop;

/// ブロック生成イベント
/// 引数にブロックタイプが指定されたらそのブロックを生成
/// なければランダムに生成
#[derive(Event, Default)]
struct BlockSpawned(Option<BlockType>);

/// ブロック固定イベント
#[derive(Event, Default)]
struct BlockFixed;

/// ブロックホールドイベント
#[derive(Event)]
struct BlockHolded(BlockType);

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
            .add_plugins(field::FieldPlugin)
            .add_plugins(block::BlockPlugin)
            .add_plugins(buttons::ButtonsPlugin)
            .add_plugins(nextblock::NextBlockPlugin)
            .add_plugins(holdblock::HoldBlockPlugin)
            .add_plugins(utils::UtilsPlugin)
            .add_plugins(scoreboard::ScoreboardPlugin)
        ;
    }
}
