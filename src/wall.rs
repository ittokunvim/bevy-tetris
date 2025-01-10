use bevy::prelude::*;

use crate::GRID_SIZE;

const WALL_THICKNESS: f32 = 1.0;
const LEFT_WALL: f32 = -5.0 * GRID_SIZE;
const RIGHT_WALL: f32 = 5.0 * GRID_SIZE;
const BOTTOM_WALL: f32 = -10.0 * GRID_SIZE;
const TOP_WALL: f32 = 10.0 * GRID_SIZE;
const WALL_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);

#[derive(Component)]
#[require(Sprite, Transform)]
struct Wall;

enum WallLocation {
    Left,
    Right,
    Bottom,
    Top,
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
            Wall,
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

pub struct WallPlugin;

impl Plugin for WallPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup)
        ;
    }
}
