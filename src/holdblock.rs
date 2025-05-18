use bevy::prelude::*;

fn setup() {
    info_once!("setup");
}

fn update(
) {
    info_once!("update");
}

pub struct HoldBlockPlugin;

impl Plugin for HoldBlockPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::InGame), setup)
            .add_systems(Update, update.run_if(in_state(AppState::InGame)))
        ;
    }
}
