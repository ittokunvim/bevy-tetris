use bevy::prelude::*;

use crate::GRID_SIZE;
use crate::ingame::{
    FIELD_SIZE,
    FIELD_POSITION,
};
use crate::ingame::blockdata::BLOCK_MAP;
use super::{
    BLOCK_SIZE,
    CurrentBlock,
    Block,
    PlayerBlock,
};

/// ブロックの落下地点を予測し描画する関数
pub fn draw_gizmos_block(
    mut gizmos: Gizmos,
    player_query: Query<&Transform, With<PlayerBlock>>,
    block_query: Query<&Transform, With<Block>>,
    current_block: Res<CurrentBlock>,
) {
    info_once!("draw_gizmos_block");

    // 衝突フラグ
    let mut collision = false;
    // 現在のステップ数（移動距離）
    let mut step = 0;

    // フィールドの下限を計算
    let field_boundary = FIELD_POSITION.y - FIELD_SIZE.y / 2.0;

    // プレイヤーブロックが衝突するか、ステップがフィールド下限に達するまで
    // プレイヤーブロックの位置の値を下に移動
    while !collision && step < BLOCK_MAP.len() {
        for player_transform in &player_query {
            let player_x = player_transform.translation.x;
            let player_y = player_transform.translation.y - GRID_SIZE * step as f32;

            for block_transform in &block_query {
                let block_x = block_transform.translation.x;
                let block_y = block_transform.translation.y;

                // プレイヤーブロックがブロックに衝突
                if player_x == block_x && player_y - GRID_SIZE == block_y {
                    collision = true;
                }
            }

            // プレイヤーブロックがフィールドの下限に衝突
            if player_y - GRID_SIZE < field_boundary {
                collision = true;
            }
        }

        // 衝突しなかったらステップを増加
        if !collision {
            step += 1;
        }
    }

    // 衝突した位置にGizmosを描画
    for player_transform in &player_query {
        // 描画するGizmosの位置を計算
        let x = player_transform.translation.x;
        let y = player_transform.translation.y - GRID_SIZE * step as f32;
        let translation = Vec2::new(x, y);

        // 描画するGizmosの幅と高さを計算
        let margin = 2.0;
        let width = BLOCK_SIZE - margin;
        let height = BLOCK_SIZE - margin;

        // 四角形のプリミティブを生成
        let primitive = Rectangle::new(width, height);

        // Gizmosの回転値
        let rotation = Rot2::radians(0.0);
        let isometry = Isometry2d::new(translation, rotation);

        // Gizmosの色を取得
        let color = current_block.blocktype.color();

        // Gizmosを使ってブロック落下地点を描画
        gizmos.primitive_2d(&primitive, isometry, color);
    }
}

