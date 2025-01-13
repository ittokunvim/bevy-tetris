use bevy::prelude::*;

use crate::GRID_SIZE;
use crate::blocks::{
    ReachBottomEvent,
    PlayerBlock,
};

const WALL_THICKNESS: f32 = 1.0;
const LEFT_WALL: f32 = -5.0 * GRID_SIZE;
const RIGHT_WALL: f32 = 5.0 * GRID_SIZE;
const BOTTOM_WALL: f32 = -10.0 * GRID_SIZE;
const TOP_WALL: f32 = 10.0 * GRID_SIZE;
const WALL_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);

#[derive(Copy, Clone)]
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
    mut events: EventWriter<ReachBottomEvent>,
    mut block_query: Query<&mut Transform, (With<PlayerBlock>, Without<Wall>)>,
    wall_query: Query<(&Wall, &Transform), (With<Wall>, Without<PlayerBlock>)>,
) {
    let mut collide_left   = false;
    let mut collide_right  = false;
    let mut collide_bottom = false;
    let mut collide_top    = false;
    // check collide
    for block_transform in &block_query {
        let (block_x, block_y) = (
            block_transform.translation.x,
            block_transform.translation.y,
        );
        for (wall, wall_transform) in &wall_query {
            let (wall_x, wall_y) = (
                wall_transform.translation.x,
                wall_transform.translation.y,
            );
            match wall.location {
                WallLocation::Left =>   if block_x <= wall_x { collide_left = true }
                WallLocation::Right =>  if block_x >= wall_x { collide_right = true }
                WallLocation::Bottom => if block_y <= wall_y { collide_bottom = true }
                WallLocation::Top =>    if block_y >= wall_y { collide_top = true}
            }
        }
    }
    // move block
    for mut block_transform in &mut block_query {
        if collide_left   { block_transform.translation.x += GRID_SIZE; }
        if collide_right  { block_transform.translation.x -= GRID_SIZE; }
        if collide_bottom { block_transform.translation.y += GRID_SIZE; }
        if collide_top    { block_transform.translation.y -= GRID_SIZE; }
    }
    // block has reach bottom
    if collide_bottom { events.send_default(); }
}

pub struct WallPlugin;

impl Plugin for WallPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup)
            // .add_systems(Update, check_for_wall) // move block.rs
        ;
    }
}
