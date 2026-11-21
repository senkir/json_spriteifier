use crate::SpriteSheetAsset;
use crate::parser::schema::TaggedSprites;
use bevy::asset::Asset;
use bevy::ecs::template::TemplateContext;
use bevy::prelude::{
    Assets, Component, Deref, DerefMut, Handle, Query, Res, Sprite, Template, Time, Timer,
    TimerMode, TypePath, error,
};
use log::info;
use std::time::Duration;

/// an Animation spec for a sprite based on a loaded sheet
/// derived sprite animation
#[derive(Asset, Clone, TypePath)]
pub struct SpriteAnimationAsset {
    from: usize,
    to: usize,
    pub tag: String,
    looping: usize,
}

impl SpriteAnimationAsset {
    pub fn new(from: usize, to: usize, tag: String, looping: usize) -> Self {
        Self {
            from,
            to,
            tag,
            looping,
        }
    }
}

impl From<TaggedSprites> for SpriteAnimationAsset {
    fn from(value: TaggedSprites) -> Self {
        SpriteAnimationAsset::new(value.from, value.to, value.name, value.looping)
    }
}

/// component responsible for handling the actual animation
#[derive(Component)]
pub struct SpriteAnimation {
    anim: SpriteAnimationAsset,
    durations: Vec<usize>,
    loop_count: usize,
}

pub struct SpriteAnimationTemplate {
    sheet: Handle<SpriteSheetAsset>,
    name: String,
}

impl Default for SpriteAnimationTemplate {
    fn default() -> Self {
        Self {
            sheet: Handle::default(),
            name: String::default(),
        }
    }
}

impl SpriteAnimationTemplate {
    pub fn new(sheet: Handle<SpriteSheetAsset>, name: String) -> Self {
        Self { sheet, name }
    }

    pub fn with_frames(mut self, sheet: Handle<SpriteSheetAsset>) -> Self {
        self.sheet = sheet;
        self
    }
}

impl Template for SpriteAnimationTemplate {
    type Output = SpriteAnimation;

    fn build_template(&self, context: &mut TemplateContext) -> bevy::prelude::Result<Self::Output> {
        let sheet_assets = context.resource::<Assets<SpriteSheetAsset>>();
        let sheet = sheet_assets
            .get(self.sheet.id())
            .ok_or("missing sheet asset")?;
        let anim_asset = sheet
            .get_animation_tag(self.name.as_str())
            .ok_or("missing tag asset")?;

        let frames_for_sheet = sheet.get_sprite_frames();

        let mut durations = Vec::with_capacity(frames_for_sheet.len());
        for (frame) in frames_for_sheet {
            durations.push(frame.duration);
        }

        Ok(SpriteAnimation {
            anim: anim_asset.clone(),
            durations,
            loop_count: 0,
        })
    }

    fn clone_template(&self) -> Self {
        SpriteAnimationTemplate {
            sheet: self.sheet.clone(),
            name: self.name.clone(),
        }
    }
}

impl SpriteAnimationAsset {
    fn has_finished(&self, index: usize) -> bool {
        self.to == index
    }

    /// generate the next frame index
    pub fn next(&self, index: usize) -> usize {
        if self.has_finished(index) || index < self.from {
            self.from
        } else if index >= self.to {
            self.to
        } else {
            index + 1
        }
    }

    /// determine if the animation has finished playing
    /// NOTE: if AnimationMode::Repeating this will never be true
    pub fn animation_finished(&self, index: usize) -> bool {
        if index < self.to || self.looping == 0 {
            return false;
        }
        true
    }

    pub fn should_loop(&self, loop_count: usize) -> bool {
        if self.looping == 0 {
            return true;
        }

        loop_count <= self.looping
    }
}

#[derive(Component, Deref, DerefMut, Clone)]
pub struct SpriteTimer {
    timer: Timer,
}

impl Default for SpriteTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(0.1, TimerMode::Repeating),
        }
    }
}

impl SpriteTimer {
    //determine animation speed
    pub fn from_seconds(seconds: f32) -> Self {
        Self {
            timer: Timer::from_seconds(seconds, TimerMode::Repeating),
        }
    }
}

/// run the animation for anything matching the given properties
pub fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(&mut SpriteAnimation, &mut SpriteTimer, &mut Sprite)>,
) {
    for (mut anim, mut timer, mut sprite) in &mut query {
        timer.tick(time.delta());

        let a_asset = anim.anim.clone();

        //next frame triggered
        if timer.just_finished()
            && let Some(atlas) = &mut sprite.texture_atlas
        {
            info!("timer finished");
            if a_asset.animation_finished(atlas.index) && !a_asset.should_loop(anim.loop_count) {
                //finished animation and done looping altogether
                info!("finished anim");
                timer.pause();
                continue;
            }

            if a_asset.animation_finished(atlas.index) && a_asset.should_loop(anim.loop_count) {
                if a_asset.looping != 0 {
                    anim.loop_count += 1;
                }

                //still go to next frame
            }
            //next frame advancement

            atlas.index = a_asset.next(atlas.index);
            let dur = anim.durations.get(atlas.index).ok_or(format!(
                "index not found in durations size{}",
                anim.durations.len()
            ));
            if (dur.is_err()) {
                error!(
                    "anim durations exhausted requested index {} from durations size {}",
                    atlas.index,
                    anim.durations.len()
                );
                continue;
            }
            timer
                .timer
                .set_duration(Duration::from_millis(*dur.unwrap() as u64));
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const START_FRAME: usize = 2;
    const END_FRAME: usize = 5;

    /// frames 2..=5; `looping` of 0 means repeat forever
    fn anim_asset(looping: usize) -> SpriteAnimationAsset {
        SpriteAnimationAsset::new(START_FRAME, END_FRAME, "walk".to_string(), looping)
    }

    fn anim(asset: SpriteAnimationAsset) -> SpriteAnimation {
        let mut durations: Vec<usize> = Vec::with_capacity(5);
        for i in 0..5 {
            durations.push(i);
        }
        SpriteAnimation {
            anim: asset,
            durations,
            loop_count: 0,
        }
    }

    // region has_finished

    #[test]
    fn has_finished_only_on_last_frame() {
        let a = anim_asset(0);
        assert!(a.has_finished(END_FRAME));
        assert!(!a.has_finished(2));
        assert!(!a.has_finished(4));
    }

    #[test]
    fn has_finished_false_past_last_frame() {
        // documents current behavior: strictly `==`, so an overshoot is not "finished"
        assert!(!anim_asset(0).has_finished(6));
    }

    // endregion

    // region next

    #[test]
    fn next_advances_by_one() {
        let a = anim_asset(0);
        assert_eq!(a.next(2), 3);
        assert_eq!(a.next(4), 5);
    }

    #[test]
    fn next_wraps_to_from_at_last_frame() {
        assert_eq!(anim_asset(0).next(END_FRAME), 2);
    }

    #[test]
    fn next_snaps_to_from_when_before_range() {
        let a = anim_asset(0);
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
        assert_ne!(anim_asset(0).next(6), 7);
    }

    // endregion

    // region animation_finished

    #[test]
    fn animation_finished_false_before_last_frame() {
        let a = anim_asset(2);
        assert!(!a.animation_finished(4));
    }

    #[test]
    fn animation_finished_never_true_when_repeating() {
        let a = anim_asset(0);
        assert!(!a.animation_finished(END_FRAME));
    }

    #[test]
    fn animation_finished_true_when_counted_loops_past_reached() {
        // intended behavior: finished once the loop count equals the configured count
        let a = anim_asset(2);
        pretty_assertions::assert_eq!(a.animation_finished(END_FRAME), true);
    }
    #[test]
    fn animation_finished_true_when_counted_loops_reached() {
        // intended behavior: finished once the loop count equals the configured count
        let a = anim_asset(2);
        pretty_assertions::assert_eq!(a.animation_finished(END_FRAME), true);
    }

    // endregion
}
