use bevy::prelude::*;

use crate::{
    GRID_SIZE,
    FIELD_SIZE,
    FIELD_POSITION,
    MoveEvent,
    Direction,
    FallingTimer,
};

const BLOCK_SIZE: f32 = GRID_SIZE - 1.0;
const BLOCK_POSITION: Vec3 = Vec3::new(
    FIELD_POSITION.x + GRID_SIZE / 2.0 - GRID_SIZE * 2.0,
    FIELD_POSITION.y + GRID_SIZE / 2.0 + FIELD_SIZE.y / 2.0 - GRID_SIZE * 1.0,
    10.0,
);
const I_BLOCK: [usize; 8]  = [
    0,0,0,0,
    1,1,1,1,
];
// const J_BLOCK: [usize; 8]  = [
//     1,0,0,0,
//     1,1,1,0,
// ];
// const L_BLOCK: [usize; 8]  = [
//     0,0,1,0,
//     1,1,1,0,
// ];
// const O_BLOCK: [usize; 8]  = [
//     0,1,1,0,
//     0,1,1,0,
// ];
// const S_BLOCK: [usize; 8]  = [
//     0,1,1,0,
//     1,1,0,0,
// ];
// const T_BLOCK: [usize; 8]  = [
//     0,1,0,0,
//     1,1,1,0,
// ];
// const Z_BLOCK: [usize; 8]  = [
//     0,1,1,0,
//     1,1,0,0,
// ];
const I_COLOR: Color = Color::srgb(0.0, 0.0, 1.0);
// const J_COLOR: Color = Color::srgb(0.0, 1.0, 0.0);
// const L_COLOR: Color = Color::srgb(0.0, 1.0, 1.0);
// const O_COLOR: Color = Color::srgb(1.0, 0.0, 0.0);
// const S_COLOR: Color = Color::srgb(1.0, 0.0, 1.0);
// const T_COLOR: Color = Color::srgb(1.0, 1.0, 0.0);
// const Z_COLOR: Color = Color::srgb(1.0, 1.0, 1.0);

#[derive(Component)]
struct Block;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let shape = meshes.add(Rectangle::new(BLOCK_SIZE, BLOCK_SIZE));
    for block in I_BLOCK.iter().enumerate() {
        let (index, value) = block;
        let (x, y, z) = (
            BLOCK_POSITION.x + GRID_SIZE * ((index % 4) as f32),
            BLOCK_POSITION.y - GRID_SIZE * ((index / 4) as f32),
            BLOCK_POSITION.z,
        );
        if *value == 0 { continue; }
        commands.spawn((
            Mesh2d(shape.clone()),
            MeshMaterial2d(materials.add(I_COLOR)),
            Transform::from_xyz(x, y, z),
            Block,
        ));
    }
}

fn block_falling(
    mut timer: ResMut<FallingTimer>,
    mut events: EventWriter<MoveEvent>,
    time: Res<Time>,
) {
    timer.tick(time.delta());
    if !timer.just_finished() { return; }
    events.send(MoveEvent(Direction::Bottom));
}

fn block_movement(
    mut events: EventReader<MoveEvent>,
    mut query: Query<&mut Transform, With<Block>>,
) {
    for event in events.read() {
        let direction = event.0;
        for mut transform in &mut query {
            match direction {
                Direction::Left   => transform.translation.x -= GRID_SIZE,
                Direction::Right  => transform.translation.x += GRID_SIZE,
                Direction::Bottom => transform.translation.y -= GRID_SIZE,
                Direction::Top    => transform.translation.y += GRID_SIZE,
            }
        }
    }
}

pub struct BlockPlugin;

impl Plugin for BlockPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup)
            .add_systems(Update, (
                block_falling,
                block_movement,
            ))
        ;
    }
}
