use bevy::prelude::*;

use crate::{
    GRID_SIZE,
    FIELD_SIZE,
    FIELD_POSITION,
    MoveEvent,
    RotationEvent,
    SpawnEvent,
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
struct PlayerBlock(usize);

#[derive(Component)]
struct Block;

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

fn setup(mut events: EventWriter<SpawnEvent>) { events.send_default(); }

fn block_spawn(
    mut events: EventReader<SpawnEvent>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut current_block: ResMut<CurrentBlock>,
    query: Query<Entity, With<PlayerBlock>>,
) {
    if events.is_empty() { return; }
    events.clear();

    for entity in &query {
        commands.entity(entity).remove::<PlayerBlock>();
        commands.entity(entity).insert(Block);
    }
    *current_block = CurrentBlock::new();
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
            PlayerBlock(*value),
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
    mut move_events: EventReader<MoveEvent>,
    mut spawn_events: EventWriter<SpawnEvent>,
    mut player_query: Query<&mut Transform, (With<PlayerBlock>, Without<Block>)>,
    mut current_block: ResMut<CurrentBlock>,
    block_query: Query<&Transform, With<Block>>,
) {
    for event in move_events.read() {
        let direction = event.0;

        for player_transform in &mut player_query {
            let player_x = player_transform.translation.x;
            let player_y = player_transform.translation.y;
            // check for field collision
            match direction {
                Direction::Left =>
                if player_x - GRID_SIZE < FIELD_POSITION.x - FIELD_SIZE.x / 2.0 { return; }
                Direction::Right =>
                if player_x + GRID_SIZE > FIELD_POSITION.x + FIELD_SIZE.x / 2.0 { return; }
                Direction::Bottom =>
                if player_y - GRID_SIZE < FIELD_POSITION.y - FIELD_SIZE.y / 2.0 {
                    spawn_events.send_default();
                    return;
                }
            }
            for block_transform in &block_query {
                let block_x = block_transform.translation.x;
                let block_y = block_transform.translation.y;
                // check for block collision
                match direction {
                    Direction::Left =>
                    if player_x - GRID_SIZE == block_x && player_y == block_y { return; }
                    Direction::Right =>
                    if player_x + GRID_SIZE == block_x && player_y == block_y { return; }
                    Direction::Bottom =>
                    if player_x == block_x && player_y - GRID_SIZE == block_y {
                        spawn_events.send_default();
                        return;
                    }
                }
            }
        }
        // updated current block position
        match direction {
            Direction::Left   => current_block.pos.x -= GRID_SIZE,
            Direction::Right  => current_block.pos.x += GRID_SIZE,
            Direction::Bottom => current_block.pos.y -= GRID_SIZE,
        }
        // moved block
        for mut transform in &mut player_query {
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
    mut timer: ResMut<FallingTimer>,
    mut query: Query<(&PlayerBlock, &mut Transform), With<PlayerBlock>>,
    mut current_block: ResMut<CurrentBlock>,
) {
    for event in events.read() {
        let direction = event.0;
        // falling timer reset
        timer.reset();
        // updated current block id
        current_block.id = match direction {
            Direction::Right => (current_block.id + 1) % MAX_BLOCKDATA,
            Direction::Left  => (current_block.id + MAX_BLOCKDATA - 1) % MAX_BLOCKDATA,
            _ => current_block.id,
        };
        for (block, mut transform) in &mut query {
            loop {
                let position = current_block.position(block.0);
                if position.x < FIELD_POSITION.x - FIELD_SIZE.x / 2.0 {
                    current_block.pos.x += GRID_SIZE;
                    continue;
                }
                else if position.x > FIELD_POSITION.x + FIELD_SIZE.x / 2.0 {
                    current_block.pos.x -= GRID_SIZE;
                    continue;
                }
                else if position.y < FIELD_POSITION.y - FIELD_SIZE.y / 2.0 {
                    current_block.pos.y += GRID_SIZE;
                    continue;
                }
                break;
            }
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
                block_spawn,
                block_falling,
                block_rotation,
                block_movement,
            ).chain())
        ;
    }
}
