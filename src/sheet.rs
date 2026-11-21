use crate::{SpriteAnimationAsset, SpriteFrameAsset, SpriteSheet};
use bevy::asset::{Asset, Assets, Handle};
use bevy::ecs::template::TemplateContext;
use bevy::image::{Image, TextureAtlas, TextureAtlasLayout};
use bevy::prelude::{Sprite, Template, TypePath};
use std::collections::HashMap;
use std::fmt::Formatter;

pub const KEY_SPRITE_SHEET: &str = "sheet";
pub const KEY_TEXTURE_LAYOUT: &str = "layout";

pub const KEY_TAG_PREFIX: &str = "t.";
pub const KEY_FRAME_PREFIX: &str = "f.";

/// Asset holding loaded data
#[derive(Asset, TypePath, Clone)]
pub struct SpriteSheetAsset {
    pub schema: SpriteSheet,
    tagged_animations: HashMap<String, SpriteAnimationAsset>,
    sprite_frames: Vec<SpriteFrameAsset>,
    pub layout: Handle<TextureAtlasLayout>,
    pub image: Option<Handle<Image>>,
}

impl std::fmt::Display for SpriteSheetAsset {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("SpriteSheet Asset")
    }
}

impl SpriteSheetAsset {
    pub fn new(
        schema: &SpriteSheet,
        layout: &Handle<TextureAtlasLayout>,
        tags: HashMap<String, SpriteAnimationAsset>,
        frames: Vec<SpriteFrameAsset>,
    ) -> Self {
        Self {
            schema: schema.clone(),
            tagged_animations: tags,
            sprite_frames: frames,
            layout: layout.clone(),
            image: None,
        }
    }

    pub fn with_image(&mut self, image: Handle<Image>) -> SpriteSheetAsset {
        self.image = Some(image);
        self.clone()
    }

    /// find matching tag
    pub fn get_animation_tag(&self, tag: &str) -> Option<&SpriteAnimationAsset> {
        self.tagged_animations.get(tag)
    }

    /// find matching frame

    pub fn get_sprite_frames(&self) -> Vec<SpriteFrameAsset> {
        self.sprite_frames.clone()
    }

    pub fn layout_key() -> String {
        KEY_TEXTURE_LAYOUT.to_string()
    }

    /// build a texture atlas for this sprite sheet
    pub fn atlas(&self, atlas_index: usize) -> TextureAtlas {
        TextureAtlas::from(self.layout.clone()).with_index(atlas_index)
    }
}

/// sprite template for use in scenes
#[derive(Debug)]
#[derive(Default)]
pub struct SpriteSheetAssetTemplate {
    pub sheet: Handle<SpriteSheetAsset>,
    pub image: Option<Handle<Image>>,
    pub index: usize,
}

impl SpriteSheetAssetTemplate {
    pub fn with_image(&mut self, image: Handle<Image>) -> SpriteSheetAssetTemplate {
        self.image = Some(image);
        self.clone_template()
    }

    pub fn with_index(&mut self, index: usize) -> SpriteSheetAssetTemplate {
        self.index = index;
        self.clone_template()
    }
}

impl From<Handle<SpriteSheetAsset>> for SpriteSheetAssetTemplate {
    fn from(value: Handle<SpriteSheetAsset>) -> Self {
        Self {
            sheet: value.clone(),
            ..Self::default()
        }
    }
}


impl Template for SpriteSheetAssetTemplate {
    type Output = Sprite;

    fn build_template(&self, ctx: &mut TemplateContext) -> bevy::prelude::Result<Self::Output> {
        let sheets = ctx.resource::<Assets<SpriteSheetAsset>>();
        let asset = sheets
            .get(self.sheet.id())
            .ok_or("sprite sheet not loaded")?;
        let img = if self.image.is_some() {
            self.image.clone()
        } else {
            asset.image.clone()
        };
        let image = img.ok_or("image not loaded in sheet")?;

        Ok(Sprite::from_atlas_image(
            image.clone(),
            TextureAtlas::from(asset.layout.clone()).with_index(self.index),
        ))
    }

    fn clone_template(&self) -> Self {
        Self {
            sheet: self.sheet.clone(),
            image: self.image.clone(),
            index: self.index,
        }
    }
}
