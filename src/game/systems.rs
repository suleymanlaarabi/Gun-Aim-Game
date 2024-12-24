use bevy::prelude::*;

use crate::resource::{HandShotGunAtlas, HandShotGunSprite};

use super::{components::InGameEntity, hand_plugin::components::HandGun};

pub fn spawn_game(
    hand_image: Res<HandShotGunSprite>,
    hand_atlas: Res<HandShotGunAtlas>,
    mut commands: Commands,
) {
    commands.spawn((
        HandGun,
        InGameEntity,
        Sprite {
            image: hand_image.0.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: hand_atlas.0.clone(),
                index: 0,
            }),
            ..default()
        },
    ));
}

pub fn despawn_game(mut commands: Commands, query: Query<Entity, With<InGameEntity>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
