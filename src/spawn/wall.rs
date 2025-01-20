use bevy::prelude::*;

use crate::AppState;
use crate::utils::prelude::*;

fn setup(
    mut commands: Commands,
) {
    commands.spawn(Wall::new(WallLocation::Left));
    commands.spawn(Wall::new(WallLocation::Right));
    commands.spawn(Wall::new(WallLocation::Bottom));
    commands.spawn(Wall::new(WallLocation::Top));
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
