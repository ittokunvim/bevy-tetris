use bevy::prelude::*;

use crate::GRID_SIZE;
use crate::block::PlayerBlock;
use crate::block::collision::BottomHitEvent as BlockBottomHitEvent;
use crate::block::movement::{
    Direction as BlockDirection,
    MoveEvent as BlockMoveEvent,
};

const WALL_THICKNESS: f32 = 1.0;
const LEFT_WALL: f32 = -5.0 * GRID_SIZE;
const RIGHT_WALL: f32 = 5.0 * GRID_SIZE;
const BOTTOM_WALL: f32 = -10.0 * GRID_SIZE;
const TOP_WALL: f32 = 10.0 * GRID_SIZE;
const WALL_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);

#[derive(Event, Default)]
pub struct CollisionEvent;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum WallLocation {
    Left,
    Right,
    Bottom,
    Top,
}

#[derive(Component)]
#[require(Sprite, Transform)]
pub struct Wall {
    pub location: WallLocation,
}

impl WallLocation {
    fn position(&self) -> Vec2 {
        match self {
            WallLocation::Left => Vec2::new(LEFT_WALL, 0.0),
            WallLocation::Right => Vec2::new(RIGHT_WALL, 0.0),
            WallLocation::Bottom => Vec2::new(0.0, BOTTOM_WALL),
            WallLocation::Top => Vec2::new(0.0, TOP_WALL),
        }
    }

    fn size(&self) -> Vec2 {
        let arena_height = TOP_WALL - BOTTOM_WALL;
        let arena_width = RIGHT_WALL - LEFT_WALL;
        assert!(arena_height > 0.0);
        assert!(arena_width > 0.0);

        match self {
            WallLocation::Left | WallLocation::Right => {
                Vec2::new(WALL_THICKNESS, arena_height + WALL_THICKNESS)
            }
            WallLocation::Bottom | WallLocation::Top => {
                Vec2::new(arena_width + WALL_THICKNESS, WALL_THICKNESS)
            }
        }
    }
}

impl Wall {
    fn new(location: WallLocation) -> (Wall, Sprite, Transform) {
        (
            Wall { location, },
            Sprite::from_color(WALL_COLOR, Vec2::ONE),
            Transform {
                translation: location.position().extend(0.0),
                scale: location.size().extend(1.0),
                ..Default::default()
            },
        )
    }
}

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
    mut write_events2: EventWriter<BlockBottomHitEvent>,
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

pub struct WallPlugin;

impl Plugin for WallPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<CollisionEvent>()
            .add_systems(Startup, setup)
            // .add_systems(Update, check_for_wall)
        ;
    }
}
