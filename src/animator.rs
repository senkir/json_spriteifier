use crate::parser::schema::FrameTag;
use bevy::asset::Asset;
use bevy::prelude::{Component, Deref, DerefMut, Query, Res, Sprite, Time, Timer, TypePath};

/// an Animation spec for a sprite based on a loaded sheet

/// derived sprite animation
#[derive(Component, Asset, Clone, TypePath)]
pub struct SpriteAnimationAsset {
    from: usize,
    to: usize,
    pub tag: String,
    looping: SpriteAnimationMode,
}

impl SpriteAnimationAsset {
    pub fn from_tag(tag: FrameTag) -> SpriteAnimationAsset {
        SpriteAnimationAsset::new(
            //TODO make a size
            tag.from,
            tag.to,
            tag.name,
            tag.looping.unwrap_or(0),
        )
    }
}

#[derive(Component, Clone, TypePath, Eq, PartialEq, Debug)]
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
            looping: a_loop,
        }
    }
}

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
        } else if (index >= self.to) {
            self.to
        } else {
            index + 1
        }
    }

    /// determine if the animation has finished playing
    /// NOTE: if AnimationMode::Repeating this will never be true
    pub fn animation_finished(&self, index: usize, loops: &Option<usize>) -> bool {
        if (index < self.to || self.looping == SpriteAnimationMode::Repeating) {
            return false;
        }

        match self.looping {
            SpriteAnimationMode::Counted(number) => {
                if (*loops >= Some(number)) {
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
    mut query: Query<(
        &SpriteAnimationAsset,
        &SpriteLoopCount,
        &mut SpriteTimer,
        &mut Sprite,
    )>,
) {
    for (indices, count, mut timer, mut sprite) in &mut query {
        timer.tick(time.delta());

        if timer.just_finished()
            && let Some(atlas) = &mut sprite.texture_atlas
        {
            if (indices.animation_finished(atlas.index, &count.0)) {
                return;
            }
            atlas.index = indices.next(atlas.index);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const START_FRAME: usize = 2;
    const END_FRAME: usize = 5;

    /// frames 2..=5; `looping` of 0 means repeat forever
    fn anim(looping: usize) -> SpriteAnimationAsset {
        SpriteAnimationAsset::new(START_FRAME, END_FRAME, "walk".to_string(), looping)
    }

    // region new

    #[test]
    fn new_zero_loops_is_repeating() {
        assert_eq!(anim(0).looping, SpriteAnimationMode::Repeating);
    }

    #[test]
    fn new_nonzero_loops_is_counted() {
        assert_eq!(anim(3).looping, SpriteAnimationMode::Counted(3));
    }

    // endregion

    // region has_finished

    #[test]
    fn has_finished_only_on_last_frame() {
        let a = anim(0);
        assert!(a.has_finished(END_FRAME));
        assert!(!a.has_finished(2));
        assert!(!a.has_finished(4));
    }

    #[test]
    fn has_finished_false_past_last_frame() {
        // documents current behavior: strictly `==`, so an overshoot is not "finished"
        assert!(!anim(0).has_finished(6));
    }

    // endregion

    // region next

    #[test]
    fn next_advances_by_one() {
        let a = anim(0);
        assert_eq!(a.next(2), 3);
        assert_eq!(a.next(4), 5);
    }

    #[test]
    fn next_wraps_to_from_at_last_frame() {
        assert_eq!(anim(0).next(END_FRAME), 2);
    }

    #[test]
    fn next_snaps_to_from_when_before_range() {
        let a = anim(0);
        assert_eq!(a.next(0), START_FRAME);
        assert_eq!(a.next(1), START_FRAME);
    }

    #[test]
    fn next_single_frame_animation_stays_put() {
        let a = SpriteAnimationAsset::new(3, 3, "idle".to_string(), 0);
        assert_eq!(a.next(3), 3);
        assert_eq!(a.next(0), 3);
    }

    #[test]
    fn next_does_not_pass_range() {
        // documents current behavior: no clamp when index > to
        assert_ne!(anim(0).next(6), 7);
    }

    // endregion

    // region animation_finished

    #[test]
    fn animation_finished_false_before_last_frame() {
        let a = anim(2);
        assert!(!a.animation_finished(4, &Some(2)));
    }

    #[test]
    fn animation_finished_never_true_when_repeating() {
        let a = anim(0);
        assert!(!a.animation_finished(END_FRAME, &Some(100)));
        assert!(!a.animation_finished(END_FRAME, &None));
    }

    #[test]
    fn animation_finished_true_when_counted_loops_past_reached() {
        // intended behavior: finished once the loop count equals the configured count
        let a = anim(2);
        pretty_assertions::assert_eq!(a.animation_finished(END_FRAME, &Some(3)), true);
    }
    #[test]
    fn animation_finished_true_when_counted_loops_reached() {
        // intended behavior: finished once the loop count equals the configured count
        let a = anim(2);
        pretty_assertions::assert_eq!(a.animation_finished(END_FRAME, &Some(2)), true);
    }

    #[test]
    fn animation_finished_false_when_counted_loops_remaining() {
        let a = anim(2);
        assert!(!a.animation_finished(END_FRAME, &Some(1)));
        assert!(!a.animation_finished(END_FRAME, &Some(0)));
    }

    #[test]
    fn animation_finished_false_when_no_loop_count_yet() {
        let a = anim(2);
        assert!(!a.animation_finished(END_FRAME, &None));
    }

    // endregion
}
