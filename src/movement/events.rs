use bevy::prelude::*;

use crate::AppState;
use super::{
    MoveEvent,
    Direction,
    BottomHitEvent,
    WallTopHitEvent,
};
use crate::utils::prelude::*;

use crate::block::SpawnEvent;

const KEY_BLOCK_LEFT_1: KeyCode = KeyCode::ArrowLeft;
const KEY_BLOCK_LEFT_2: KeyCode = KeyCode::KeyA;
const KEY_BLOCK_RIGHT_1: KeyCode = KeyCode::ArrowRight;
const KEY_BLOCK_RIGHT_2: KeyCode = KeyCode::KeyD;

pub fn movement(
    mut events: EventWriter<MoveEvent>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    let mut closure = |direction: Direction| {
        // debug!("block_movement: send event");
        events.send(MoveEvent(direction));
    };

    for key in keyboard_input.get_just_pressed() {
        match key {
            &KEY_BLOCK_LEFT_1  | &KEY_BLOCK_LEFT_2  => closure(Direction::Left),
            &KEY_BLOCK_RIGHT_1 | &KEY_BLOCK_RIGHT_2 => closure(Direction::Right),
            _ => {},
        }
    }
}

pub fn bottom_hit(
    mut read_events: EventReader<BottomHitEvent>,
    mut write_events: EventWriter<SpawnEvent>,
    mut commands: Commands,
    mut current_block: ResMut<CurrentBlock>,
    query: Query<Entity, With<PlayerBlock>>,
) {
    if read_events.is_empty() { return }
    read_events.clear();
    // debug!("reset current block");
    *current_block = CurrentBlock::reset();
    // debug!("remove PlayerBlock components");
    for entity in &query { commands.entity(entity).remove::<PlayerBlock>(); }
    // debug!("send spawn event");
    write_events.send_default();
}

pub fn top_hit(
    mut read_events: EventReader<WallTopHitEvent>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if read_events.is_empty() { return }
    read_events.clear();
    // trace!("AppState::Ingame -> Gameover");
    next_state.set(AppState::Gameover);
}
