use animation_plugin::{event::AnimationEvent, Animation2d};
use bevy::{
    input::{mouse::MouseButtonInput, ButtonState},
    prelude::*,
    window::PrimaryWindow,
};

use crate::resource::{ContactSheet, ContactSheetAtlas, ShootAudio};

use super::components::{CrossHairCircle, HandGun};

pub fn update_hand_position(
    mut query: Query<&mut Transform, With<HandGun>>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
) {
    if let Some(position) = q_windows.single().cursor_position() {
        let mut transform = query.single_mut();
        transform.translation = Vec3::new(position.x - 500., -(position.y + 210.), 1.);
    }
}

pub fn on_crosshair_animaiton_end(
    q_cross_hair: Query<Entity, With<CrossHairCircle>>,
    mut event_reader: EventReader<AnimationEvent>,
    mut commands: Commands,
) {
    for ev in event_reader.read() {
        for entity in &q_cross_hair {
            if ev.0 == entity {
                commands.entity(entity).despawn();
            }
        }
    }
}

pub fn handle_shoot(
    mut query: Query<Entity, With<HandGun>>,
    mut mouse_evt: EventReader<MouseButtonInput>,
    audio: Res<ShootAudio>,
    mut commands: Commands,
    contact_sheet: Res<ContactSheet>,
    contact_sheet_atlas: Res<ContactSheetAtlas>,
) {
    for ev in mouse_evt.read() {
        match ev.state {
            ButtonState::Pressed => {
                if ev.button == MouseButton::Left {
                    commands.spawn(AudioPlayer::new(audio.0.clone()));
                    let entity = query.single_mut();
                    commands.entity(entity).with_children(|parent| {
                        parent.spawn((
                            CrossHairCircle,
                            Sprite {
                                texture_atlas: Some(TextureAtlas {
                                    index: 0,
                                    layout: contact_sheet_atlas.0.clone(),
                                }),
                                image: contact_sheet.0.clone(),
                                ..default()
                            },
                            Animation2d::new(0.07, 0, 3),
                            Transform::from_xyz(-220., 820., 2.),
                        ));
                    });
                }
            }
            _ => {}
        }
    }
}
