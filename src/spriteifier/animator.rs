use bevy::prelude::{Component, Deref, DerefMut, Query, Res, Time, Timer};

#[derive(Component, Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum AnimationMode {
    Repeating,
    Once,
}

#[derive(Component)]
pub struct AnimationIndices {
    first: usize,
    last: usize,
    repeating: AnimationMode,
}

impl AnimationIndices {
    pub fn new(first: usize, last: usize, repeating: AnimationMode) -> Self {
        Self {
            first,
            last,
            repeating,
        }
    }
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer {
    timer: Timer,
}
/// run the animation for anything matching the given properties
pub fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(&AnimationIndices, &mut AnimationTimer, &mut Sprite)>,
) {
    for (indices, mut timer, mut sprite) in &mut query {
        timer.tick(time.delta());

        if timer.just_finished()
            && let Some(atlas) = &mut sprite.texture_atlas
        {
            if (indices.animation_finished(atlas.index)) {
                return;
            }
            atlas.index = indices.next(atlas.index);
        }
    }
}