use bevy::asset::Asset;
use bevy::prelude::{Component, Deref, DerefMut, Query, Res, Sprite, Time, Timer, TypePath};

/// an Animation spec for a sprite based on a loaded sheet

/// derived sprite animation
#[derive(Component, Asset, Clone, TypePath)]
pub struct SpriteAnimationAsset {
    from: usize,
    to: usize,
    tag: String,
    looping: SpriteAnimationMode,
}

#[derive(Component, Clone, TypePath, Eq, PartialEq)]
pub enum SpriteAnimationMode {
    Repeating,
    Counted(usize),
}

/// marker component for number of animation loops that have taken place
#[derive(Component, Clone, TypePath)]
pub struct SpriteLoopCount(pub Option<usize>);

impl SpriteAnimationAsset {
    pub fn new(from: usize, to: usize, tag: String, looping: usize) -> SpriteAnimationAsset {
        let a_loop = if (looping == 0) {
            SpriteAnimationMode::Repeating
        } else {
            SpriteAnimationMode::Counted(looping)
        };
        SpriteAnimationAsset {
            from,
            to,
            tag,
            looping: a_loop
        }
    }
}

pub fn load_animation() {}

impl SpriteAnimationAsset {
    fn has_finished(&self, index: usize) -> bool {
        self.to == index
    }

    /// generate the next frame index
    pub fn next(&self, index: usize) -> usize {
        if self.has_finished(index) {
            self.from
        } else if (index < self.from) {
            self.from
        } else {
            index + 1
        }
    }

    /// determine if the animation has finished playing
    /// NOTE: if AnimationMode::Repeating this will never be true
    pub fn animation_finished(&self, index: usize, loops: &SpriteLoopCount) -> bool {
        if (index < self.to) {
            return false;
        }
        
        match self.looping {
            SpriteAnimationMode::Repeating => {
                false
            }
            SpriteAnimationMode::Counted(n) => {
                if (loops.0 == Some(index)) {
                    true
                } else {
                    false
                }
            }
            _ => {
                //fallback
                false
            }
        }
    }
}

#[derive(Component, Deref, DerefMut)]
pub struct SpriteTimer {
    timer: Timer,
}

/// run the animation for anything matching the given properties
pub fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(&SpriteAnimationAsset, &SpriteLoopCount, &mut SpriteTimer, &mut Sprite)>,
) {
    for (indices, count, mut timer, mut sprite) in &mut query {
        timer.tick(time.delta());

        if timer.just_finished()
            && let Some(atlas) = &mut sprite.texture_atlas
        {
            if (indices.animation_finished(atlas.index, count)) {
                return;
            }
            atlas.index = indices.next(atlas.index);
        }
    }
}
