use bevy::prelude::*;

#[derive(Resource)]
pub struct HandShotGunSprite(pub Handle<Image>);

#[derive(Resource)]
pub struct ContactSheet(pub Handle<Image>);

#[derive(Resource)]
pub struct ContactSheetAtlas(pub Handle<TextureAtlasLayout>);

#[derive(Resource)]
pub struct HandShotGunAtlas(pub Handle<TextureAtlasLayout>);

#[derive(Resource)]
pub struct ShootAudio(pub Handle<AudioSource>);

pub struct AppResourcePlugin;

impl Plugin for AppResourcePlugin {
    fn build(&self, app: &mut App) {
        let world = app.world_mut();
        init_resource_from_world(world);
    }
}

fn init_resource_from_world(world: &mut World) {
    let server = world.resource_mut::<AssetServer>();
    // LOAD HANDLE
    let hand_image: Handle<Image> = server.load("sprites/m4_sheet.png");
    let atlas_handle: Handle<TextureAtlasLayout> = server.add(TextureAtlasLayout::from_grid(
        UVec2::new(1550, 1550),
        3,
        1,
        None,
        None,
    ));
    let contact_sheet_atlas: Handle<TextureAtlasLayout> = server.add(
        TextureAtlasLayout::from_grid(UVec2::new(48, 48), 4, 1, None, None),
    );
    let handle_shoot_audio: Handle<AudioSource> = server.load("sounds/M4.ogg");
    let handle_contact_sheet: Handle<Image> = server.load("sprites/contact_sheet.png");
    // INSERT RESOURCE
    world.insert_resource(HandShotGunSprite(hand_image));
    world.insert_resource(HandShotGunAtlas(atlas_handle));
    world.insert_resource(ShootAudio(handle_shoot_audio));
    world.insert_resource(ContactSheet(handle_contact_sheet));
    world.insert_resource(ContactSheetAtlas(contact_sheet_atlas));
}
