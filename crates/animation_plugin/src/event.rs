use bevy::prelude::*;

use crate::Animation2d;

#[derive(Event)]
pub struct AnimationEvent(pub Entity);

pub struct CustomAnimationEventPlugin;

impl Plugin for CustomAnimationEventPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<AnimationEvent>()
            .add_systems(Update, on_animation_update);
    }
}
fn on_animation_update(
    query: Query<(Entity, &Animation2d, &Sprite)>,
    mut event_writer: EventWriter<AnimationEvent>,
) {
    for (entity, animation, sprite) in &query {
        if let Some(atlas) = &sprite.texture_atlas {
            if atlas.index == animation.indices.last {
                event_writer.send(AnimationEvent(entity));
            }
        }
    }
}
