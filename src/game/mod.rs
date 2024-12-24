use bevy::prelude::*;
use hand_plugin::HandGunPlugin;
use systems::{despawn_game, spawn_game};

use crate::GameState;

mod ball_plugin;
mod components;
mod hand_plugin;
mod systems;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(HandGunPlugin)
            .add_systems(OnEnter(GameState::InGame), spawn_game)
            .add_systems(OnExit(GameState::InGame), despawn_game);
    }
}
