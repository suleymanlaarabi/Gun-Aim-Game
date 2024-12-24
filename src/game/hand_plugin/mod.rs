use bevy::prelude::*;
use systems::{handle_shoot, on_crosshair_animaiton_end, update_hand_position};

pub mod components;
mod systems;

pub struct HandGunPlugin;

impl Plugin for HandGunPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                update_hand_position,
                handle_shoot,
                on_crosshair_animaiton_end,
            ),
        );
    }
}
