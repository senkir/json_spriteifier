pub mod spriteifier {
    use crate::animator::{SpriteAnimationTemplate, SpriteTimer};
    use crate::{SpriteSheetAsset, SpriteSheetAssetTemplate, sheet};
    use bevy::asset::Handle;
    use bevy::image::Image;

    pub fn sprite_sheet(
        sheet: Handle<SpriteSheetAsset>,
        image: Handle<Image>,
    ) -> SpriteSheetAssetTemplate {
        SpriteSheetAssetTemplate::from(sheet).with_image(image)
    }

    pub fn sprite_animation(
        sheet: Handle<SpriteSheetAsset>,
        name: &str,
    ) -> SpriteAnimationTemplate {
        SpriteAnimationTemplate::new(sheet, String::from(name))
    }

    pub fn sheet_timer() -> SpriteTimer {
        SpriteTimer::from_seconds(0.1)
    }

    /// tag asset name for tag name
    pub fn tag_name(name: &str) -> String {
        let prefix = sheet::KEY_TAG_PREFIX;
        format!("{prefix}{name}")
    }

    // frame asset name for frame name
    pub fn frame_name(name: &str) -> String {
        let prefix = sheet::KEY_FRAME_PREFIX;
        format!("{prefix}{name}")
    }
}
