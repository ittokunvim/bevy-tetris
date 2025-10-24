use bevy::prelude::*;

use crate::ingame::{
    Direction,
    MoveEvent,
    RotationEvent,
    HardDropEvent,
    HoldEvent,
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
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<MoveLeftButton>)>,
    mut events: EventWriter<MoveEvent>,
) {
    info_once!("button_block_moveleft");

    for interaction in &mut interaction_query {
        if let Interaction::Pressed = *interaction {
            events.send(MoveEvent(Direction::Left));
        }
    }
}

pub fn button_block_moveright(
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<MoveRightButton>)>,
    mut events: EventWriter<MoveEvent>,
) {
    info_once!("button_block_moveright");

    for interaction in &mut interaction_query {
        if let Interaction::Pressed = *interaction {
            events.send(MoveEvent(Direction::Right));
        }
    }
}

pub fn button_block_movebottom(
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<MoveBottomButton>)>,
    mut events: EventWriter<MoveEvent>,
) {
    info_once!("button_block_movebottom");

    for interaction in &mut interaction_query {
        if let Interaction::Pressed = *interaction {
            events.send(MoveEvent(Direction::Bottom));
        }
    }
}

pub fn button_block_rotationleft(
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<RotateLeftButton>)>,
    mut events: EventWriter<RotationEvent>,
) {
    info_once!("button_block_rotationleft");

    for interaction in &mut interaction_query {
        if let Interaction::Pressed = *interaction {
            events.send(RotationEvent(Direction::Left));
        }
    }
}

pub fn button_block_rotationright(
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<RotateRightButton>)>,
    mut events: EventWriter<RotationEvent>,
) {
    info_once!("button_block_rotationright");

    for interaction in &mut interaction_query {
        if let Interaction::Pressed = *interaction {
            events.send(RotationEvent(Direction::Right));
        }
    }
}

pub fn button_block_harddrop(
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<HarddropButton>)>,
    mut events: EventWriter<HardDropEvent>,
) {
    info_once!("button_block_harddrop");

    for interaction in &mut interaction_query {
        if let Interaction::Pressed = *interaction {
            events.send_default();
        }
    }
}

pub fn button_block_hold(
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<HoldButton>)>,
    mut events: EventWriter<HoldEvent>,
    mut holdblocks: ResMut<HoldBlocks>,
    currentblock: Res<CurrentBlocks>,
) {
    info_once!("button_block_hold");

    for interaction in &mut interaction_query {
        if let Interaction::Pressed = *interaction {
            if holdblocks.can_hold {
                holdblocks.can_hold = false;
                events.send(HoldEvent(currentblock.blocktype));
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

