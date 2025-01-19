use bevy::prelude::*;

use crate::{
    GRID_SIZE,
    AppState,
};
use crate::block::{
    MoveEvent as BlockMoveEvent,
    Direction as BlockDirection,
    SpawnEvent,
};
use crate::utils::prelude::*;

fn setup(
    mut commands: Commands,
) {
    commands.spawn(Wall::new(WallLocation::Left));
    commands.spawn(Wall::new(WallLocation::Right));
    commands.spawn(Wall::new(WallLocation::Bottom));
    commands.spawn(Wall::new(WallLocation::Top));
}

pub fn check_for_wall(
    mut read_events: EventReader<BlockMoveEvent>,
    mut write_events1: EventWriter<CollisionEvent>,
    mut write_events2: EventWriter<BottomHitEvent>,
    mut write_events3: EventWriter<TopHitEvent>,
    player_query: Query<&Transform, (With<PlayerBlock>, Without<Wall>)>,
    wall_query: Query<(&Wall, &Transform), (With<Wall>, Without<PlayerBlock>)>,
) {
    for block_move_event in read_events.read() {
        let direction = block_move_event.0;
        // send event closure
        let mut closure = |location: WallLocation| {
            // trace!("location: {:?}", location);
            write_events1.send_default();
            if location == WallLocation::Bottom { write_events2.send_default(); }
            if location == WallLocation::Top    { write_events3.send_default(); }
        };
        // check collision wall
        for player_transform in &player_query {
            let (mut player_x, mut player_y) = (
                player_transform.translation.x,
                player_transform.translation.y,
            );
            match direction {
                BlockDirection::Left   => player_x -= GRID_SIZE,
                BlockDirection::Right  => player_x += GRID_SIZE,
                BlockDirection::Bottom => player_y -= GRID_SIZE,
            }
            for (wall, wall_transform) in &wall_query {
                let (wall_x, wall_y) = (
                    wall_transform.translation.x,
                    wall_transform.translation.y,
                );
                match wall.location {
                    WallLocation::Left =>
                    if player_x <= wall_x { closure(wall.location) }
                    WallLocation::Right =>
                    if player_x >= wall_x { closure(wall.location) }
                    WallLocation::Bottom =>
                    if player_y <= wall_y { closure(wall.location) }
                    WallLocation::Top =>
                    if player_y >= wall_y { closure(wall.location) }
                }
            }
        }
    }
}

fn despawn_all(
    mut commands: Commands,
    query: Query<Entity, With<Wall>>,
) {
    // debug!("despawn_all");
    for entity in &query { commands.entity(entity).despawn() }
}

pub struct WallPlugin;

impl Plugin for WallPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::Ingame), setup)
            // .add_systems(Update, check_for_wall)
            .add_systems(OnExit(AppState::Ingame), despawn_all)
        ;
    }
}
