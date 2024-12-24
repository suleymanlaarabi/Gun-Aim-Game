use bevy::prelude::*;

fn default_gun_transform() -> Transform {
    let mut transform = Transform::from_xyz(0., 0., 0.);
    transform.scale = Vec3::splat(0.75);
    transform
}

#[derive(Component)]
#[require(Transform(default_gun_transform), Sprite)]
pub struct HandGun;

#[derive(Component)]
pub struct CrossHairCircle;
