use bevy::prelude::*;

use crate::{
    GRID_SIZE,
    FIELD_SIZE,
    FIELD_POSITION,
    MoveEvent,
    RotationEvent,
    Direction,
    FallingTimer,
};
use crate::blockdata::{
    MAX_BLOCKDATA,
    I_BLOCK,
    I_COLOR,
};

const BLOCK_SIZE: f32 = GRID_SIZE - 1.0;
const BLOCK_POSITION: Vec3 = Vec3::new(
    FIELD_POSITION.x + GRID_SIZE / 2.0 - GRID_SIZE * 2.0,
    FIELD_POSITION.y + GRID_SIZE / 2.0 + FIELD_SIZE.y / 2.0 - GRID_SIZE * 1.0,
    10.0,
);

#[derive(Resource)]
struct CurrentBlock {
    id: usize,
    pos: Vec3,
}

#[derive(Component)]
struct Block(usize);

impl CurrentBlock {
    fn new() -> Self {
        CurrentBlock {
            id: 0,
            pos: BLOCK_POSITION,
        }
    }

    fn position(&self, id: usize) -> Vec3 {
        let current_id = if self.id < I_BLOCK.len() { self.id }
        else { panic!("self.id is too long: {}", self.id); };

        for block in I_BLOCK[current_id].iter().enumerate() {
            let (index, value) = block;
            if id == *value {
                let (x, y, z) = (
                    self.pos.x + GRID_SIZE * ((index % 4) as f32),
                    self.pos.y - GRID_SIZE * ((index / 4) as f32),
                    self.pos.z,
                );
                return Vec3::new(x, y, z);
            }
        }
        panic!("id not found: {}", id);
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let shape = meshes.add(Rectangle::new(BLOCK_SIZE, BLOCK_SIZE));
    for block in I_BLOCK[0].iter().enumerate() {
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
            Block(*value),
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
    mut current_block: ResMut<CurrentBlock>,
) {
    for event in events.read() {
        let direction = event.0;

        match direction {
            Direction::Left   => current_block.pos.x -= GRID_SIZE,
            Direction::Right  => current_block.pos.x += GRID_SIZE,
            Direction::Bottom => current_block.pos.y -= GRID_SIZE,
        }
        for mut transform in &mut query {
            match direction {
                Direction::Left   => transform.translation.x -= GRID_SIZE,
                Direction::Right  => transform.translation.x += GRID_SIZE,
                Direction::Bottom => transform.translation.y -= GRID_SIZE,
            }
        }
    }
}

fn block_rotation(
    mut events: EventReader<RotationEvent>,
    mut query: Query<(&Block, &mut Transform), With<Block>>,
    mut current_block: ResMut<CurrentBlock>,
) {
    for event in events.read() {
        let direction = event.0;
        
        if direction == Direction::Right {
            let id = current_block.id;
            current_block.id = if id + 1 < MAX_BLOCKDATA { id + 1 } else { 0 };
        }
        for (block, mut transform) in &mut query {
            transform.translation = current_block.position(block.0);
        }
    }
}

pub struct BlockPlugin;

impl Plugin for BlockPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(CurrentBlock::new())
            .add_systems(Startup, setup)
            .add_systems(Update, (
                block_falling,
                block_movement,
                block_rotation,
            ))
        ;
    }
}
