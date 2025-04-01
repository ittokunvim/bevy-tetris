use bevy::prelude::*;

const GAMETITLE: &str = "テトリス";
const WINDOW_SIZE: Vec2 = Vec2::new(640.0, 480.0);
const BACKGROUND_COLOR: Color = Color::srgb(0.1, 0.1, 0.1);
const PATH_SOUND_BGM: &str = "bevy-tetris/bgm.ogg";

const GRID_SIZE: f32 = 20.0;
const FIELD_SIZE: Vec2 = Vec2::new(10.0 * GRID_SIZE, 20.0 * GRID_SIZE);
const FIELD_COLOR: Color = Color::srgb(0.6, 0.6, 0.6);
const FIELD_POSITION: Vec3 = Vec3::new(0.0, 0.0, -10.0);

#[derive(Component)]
struct Field;

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
        .add_systems(Startup, setup)
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
}
