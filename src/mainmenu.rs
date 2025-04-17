use bevy::prelude::*;

fn setup(
) {
}

fn update(
) {
}

fn despawn(
    mut commands: Commands,
    query: Query<Entity, With<Mainmenu>>,
) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

pub struct MainmenuPlugin;

impl Plugin for MainmenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::Mainmenu), setup)
            .add_systems(Update, update.run_if(in_state(AppState::Mainmenu)))
            .add_systems(OnExit(AppState::Mainmenu), despawn)
        ;
    }
}
