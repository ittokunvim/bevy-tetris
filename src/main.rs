use bevy::prelude::*;

const GAMETITLE: &str = "テトリス";
const WINDOW_SIZE: Vec2 = Vec2::new(640.0, 480.0);
const BACKGROUND_COLOR: Color = Color::srgb(0.1, 0.1, 0.1);
const PATH_SOUND_BGM: &str = "bevy-tetris/bgm.ogg";

const GRID_SIZE: f32 = 20.0;

const FIELD_SIZE: Vec2 = Vec2::new(10.0 * GRID_SIZE, 20.0 * GRID_SIZE);
const FIELD_COLOR: Color = Color::srgb(0.6, 0.6, 0.6);
const FIELD_POSITION: Vec3 = Vec3::new(0.0, 0.0, -10.0);

const BLOCK_SIZE: f32 = GRID_SIZE - 1.0;
const BLOCK_POSITION: Vec3 = Vec3::new(
    FIELD_POSITION.x + GRID_SIZE / 2.0 - GRID_SIZE * 2.0,
    FIELD_POSITION.y + GRID_SIZE / 2.0 + FIELD_SIZE.y / 2.0 - GRID_SIZE * 1.0,
    10.0,
);
const BLOCK_SPEED: f32 = 0.5;
const I_BLOCK: [usize; 8]  = [
    0,0,0,0,
    1,1,1,1,
];
const J_BLOCK: [usize; 8]  = [
    1,0,0,0,
    1,1,1,0,
];
const L_BLOCK: [usize; 8]  = [
    0,0,1,0,
    1,1,1,0,
];
const O_BLOCK: [usize; 8]  = [
    0,1,1,0,
    0,1,1,0,
];
const S_BLOCK: [usize; 8]  = [
    0,1,1,0,
    1,1,0,0,
];
const T_BLOCK: [usize; 8]  = [
    0,1,0,0,
    1,1,1,0,
];
const Z_BLOCK: [usize; 8]  = [
    0,1,1,0,
    1,1,0,0,
];
const I_COLOR: Color = Color::srgb(0.0, 0.0, 1.0);
const J_COLOR: Color = Color::srgb(0.0, 1.0, 0.0);
const L_COLOR: Color = Color::srgb(0.0, 1.0, 1.0);
const O_COLOR: Color = Color::srgb(1.0, 0.0, 0.0);
const S_COLOR: Color = Color::srgb(1.0, 0.0, 1.0);
const T_COLOR: Color = Color::srgb(1.0, 1.0, 0.0);
const Z_COLOR: Color = Color::srgb(1.0, 1.0, 1.0);

#[derive(Event)]
struct MoveEvent(Direction);

#[derive(Copy, Clone)]
enum Direction {
    Left,
    Right,
    Top,
    Bottom,
}

#[derive(Resource, Deref, DerefMut)]
struct FallingTimer(Timer);

#[derive(Component)]
struct Field;

#[derive(Component)]
struct Block;

impl FallingTimer {
    fn new() -> Self {
        Self(Timer::from_seconds(BLOCK_SPEED, TimerMode::Repeating))
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    resolution: WINDOW_SIZE.into(),
                    title: GAMETITLE.to_string(),
                    ..Default::default()
                }),
                ..Default::default()
            })
        )
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .insert_resource(Time::<Fixed>::from_seconds(1.0 / 60.0))
        .add_event::<MoveEvent>()
        .insert_resource(FallingTimer::new())
        .add_systems(Startup, setup)
        .add_systems(Update, (
            send_falling_event,
            block_movement,
        ))
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // camera
    commands.spawn(Camera2d::default());
    // bgm
    let sound = AudioPlayer::new(asset_server.load(PATH_SOUND_BGM));
    let settings = PlaybackSettings::LOOP;
    commands.spawn((sound, settings));
    // field
    let shape = meshes.add(Rectangle::new(FIELD_SIZE.x, FIELD_SIZE.y));
    commands.spawn((
        Mesh2d(shape),
        MeshMaterial2d(materials.add(FIELD_COLOR)),
        Transform::from_xyz(FIELD_POSITION.x, FIELD_POSITION.y, FIELD_POSITION.z),
        Field,
    ));
    // block
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

fn send_falling_event(
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
