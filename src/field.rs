use bevy::prelude::*;

use crate::{
    FIELD_SIZE,
    FIELD_POSITION,
    AppState,
};

const FIELD_COLOR: Color = Color::srgb(0.6, 0.6, 0.6);

#[derive(Component)]
struct Field;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // field
    let shape = meshes.add(Rectangle::new(FIELD_SIZE.x, FIELD_SIZE.y));
    commands.spawn((
        Mesh2d(shape),
        MeshMaterial2d(materials.add(FIELD_COLOR)),
        Transform::from_xyz(FIELD_POSITION.x, FIELD_POSITION.y, FIELD_POSITION.z),
        Field,
    ));
}

pub struct FieldPlugin;

impl Plugin for FieldPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::InGame), setup)
        ;
    }
}
