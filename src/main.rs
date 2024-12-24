use animation_plugin::{event::CustomAnimationEventPlugin, CustomAnimationPlugin};
use avian2d::prelude::*;
use bevy::{prelude::*, window::CursorOptions};
use game::GamePlugin;
use resource::AppResourcePlugin;
use setup::AppSetupPlugin;

mod game;
mod resource;
mod setup;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    Menu,
    InGame,
    Loose,
}

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "First Person Hopper".into(),
                        resolution: (1280., 720.).into(),
                        resizable: false,
                        cursor_options: CursorOptions {
                            visible: false,
                            ..default()
                        },
                        ..default()
                    }),
                    ..default()
                })
                .build(),
        )
        .insert_state(GameState::InGame)
        .add_plugins((
            // PHYSICS
            PhysicsPlugins::default(),
            PhysicsDebugPlugin::default(),
        ))
        // APP PLUGIN
        .add_plugins((
            AppSetupPlugin,
            AppResourcePlugin,
            GamePlugin,
            CustomAnimationPlugin,
            CustomAnimationEventPlugin,
        ))
        .run();
}
