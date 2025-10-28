use bevy::prelude::*;

use crate::ingame::{
    Direction,
    BlockMoved,
    BlockRotated,
    BlockHarddrop,
    BlockHolded,
};

use crate::ingame::utils::{
    CurrentBlocks,
    HoldBlocks,
};

use super::{
    KeyButton,
    HarddropButton,
    HoldButton,
    MoveBottomButton,
    MoveLeftButton,
    MoveRightButton,
    RotateLeftButton,
    RotateRightButton,
};

pub fn button_block_moveleft(
    mut commands: Commands,
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<MoveLeftButton>)>,
) {
    info_once!("button_block_moveleft");

    for interaction in &mut interaction_query {
        if let Interaction::Pressed = *interaction {
            commands.trigger(BlockMoved(Direction::Left));
        }
    }
}

pub fn button_block_moveright(
    mut commands: Commands,
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<MoveRightButton>)>,
) {
    info_once!("button_block_moveright");

    for interaction in &mut interaction_query {
        if let Interaction::Pressed = *interaction {
            commands.trigger(BlockMoved(Direction::Right));
        }
    }
}

pub fn button_block_movebottom(
    mut commands: Commands,
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<MoveBottomButton>)>,
) {
    info_once!("button_block_movebottom");

    for interaction in &mut interaction_query {
        if let Interaction::Pressed = *interaction {
            commands.trigger(BlockMoved(Direction::Bottom));
        }
    }
}

pub fn button_block_rotationleft(
    mut commands: Commands,
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<RotateLeftButton>)>,
) {
    info_once!("button_block_rotationleft");

    for interaction in &mut interaction_query {
        if let Interaction::Pressed = *interaction {
            commands.trigger(BlockRotated(Direction::Left));
        }
    }
}

pub fn button_block_rotationright(
    mut commands: Commands,
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<RotateRightButton>)>,
) {
    info_once!("button_block_rotationright");

    for interaction in &mut interaction_query {
        if let Interaction::Pressed = *interaction {
            commands.trigger(BlockRotated(Direction::Right));
        }
    }
}

pub fn button_block_harddrop(
    mut commands: Commands,
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<HarddropButton>)>,
) {
    info_once!("button_block_harddrop");

    for interaction in &mut interaction_query {
        if let Interaction::Pressed = *interaction {
            commands.trigger(BlockHarddrop);
        }
    }
}

pub fn button_block_hold(
    mut commands: Commands,
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<HoldButton>)>,
    mut holdblocks: ResMut<HoldBlocks>,
    currentblock: Res<CurrentBlocks>,
) {
    info_once!("button_block_hold");

    for interaction in &mut interaction_query {
        if let Interaction::Pressed = *interaction {
            if holdblocks.can_hold {
                holdblocks.can_hold = false;
                commands.trigger(BlockHolded(currentblock.blocktype));
            }
        }
    }
}

pub fn despawn(
    mut commands: Commands,
    query: Query<Entity, With<KeyButton>>,
) {
    info_once!("despawn");

    for entity in &query {
        commands.entity(entity).despawn();
    }
}

