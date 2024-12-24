use bevy::prelude::*;

pub struct AppSetupPlugin;

impl Plugin for AppSetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera);
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}
