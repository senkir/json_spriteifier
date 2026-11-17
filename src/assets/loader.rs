use std::collections::HashMap;
use crate::animator::SpriteAnimationAsset;
use crate::frame::SpriteFrameAsset;
use crate::parser::schema::SpriteSheet;
use bevy::asset::io::Reader;
use bevy::asset::{Asset, AssetLoader, LoadContext};
use bevy::image::TextureAtlasLayout;
use bevy::prelude::{Handle, TypePath};
use serde_json::from_slice;

/// supported extensions for the loader
const EXTENSIONS: [&str; 1] = ["json"];

const KEY_SPRITE_SHEET: &str = "sheet";
const KEY_TEXTURE_LAYOUT: &str = "layout";

const KEY_TAG_PREFIX: &str = "t_";
const KEY_FRAME_PREFIX: &str = "f_";

/// Asset holding loaded data
#[derive(Asset, TypePath)]
pub struct SpriteSheetAsset {
    pub schema: Handle<SpriteSheet>,
    tags: HashMap<String,Handle<SpriteAnimationAsset>>,
    frames: HashMap<String, Handle<SpriteFrameAsset>>,
    pub layout: Handle<TextureAtlasLayout>,
}

impl SpriteSheetAsset {
    pub fn tags(&self, tag: &str) -> Option<&Handle<SpriteAnimationAsset>> { self.tags.get(tag) }
    pub fn frame(&self, name: &str) -> Option<&Handle<SpriteFrameAsset>> { self.frames.get(name) }
}

// region loader

/// Loader capable of handling sprite sheet json data
#[derive(Default, TypePath)]
pub struct SpriteJsonLoader;

impl SpriteJsonLoader {
    pub const SUPPORTED_FORMATS: [&'static str; 1] = ["json"];
    pub fn new() -> SpriteJsonLoader {
        SpriteJsonLoader {}
    }

    fn load_asset_layout(&self, sprite_schema: &SpriteSheet) -> TextureAtlasLayout {
        let mut layout = TextureAtlasLayout::new_empty(sprite_schema.sheet_size());
        for frame in sprite_schema.frames.iter() {
            layout.add_texture(frame.rect());
        }
        layout
    }

    fn load_animation_tags(
        &self,
        sprites: &SpriteSheet,
        load_context: &mut LoadContext,
    ) -> HashMap<String, Handle<SpriteAnimationAsset>> {
        let mut out = HashMap::with_capacity(sprites.meta.frame_tags.len());
        for tag in sprites.meta.frame_tags.iter() {
            let anim_asset = SpriteAnimationAsset::from_tag(tag.clone());
            out.insert( anim_asset.tag.clone(), load_context.add_labeled_asset(
                format!("{}{}", KEY_TAG_PREFIX, anim_asset.tag.clone()),
                anim_asset.clone(),
            ));
        }
        out
    }

    fn load_indices(
        &self,
        sprites: &SpriteSheet,
        load_context: &mut LoadContext<'_>,
    ) -> HashMap<String, Handle<SpriteFrameAsset>> {
        let mut out = HashMap::with_capacity(sprites.frames.len());
        for (i, frame) in sprites.frames.iter().enumerate() {
            if (frame.name == None) {
                continue;
            }
            let name = frame.name.clone().unwrap();
            let asset = load_context.add_labeled_asset(
                format!("{}{}",KEY_FRAME_PREFIX, name.clone()),
                SpriteFrameAsset {
                    frame: frame.rect(),
                    index: i,
                },
            );
            out.insert(name, asset);
        }
        out
    }
}

impl AssetLoader for SpriteJsonLoader {
    type Asset = SpriteSheetAsset;

    type Settings = ();
    type Error = Box<dyn std::error::Error + Send + Sync>;

    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &Self::Settings,
        load_context: &mut LoadContext<'_>,
    ) -> Result<SpriteSheetAsset, Self::Error> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;
        let sprite_schema = from_slice::<SpriteSheet>(&bytes)?;

        let non_grid_layout = self.load_asset_layout(&sprite_schema);
        let handle = load_context.add_labeled_asset(KEY_TEXTURE_LAYOUT, non_grid_layout);

        let tag_handles = self.load_animation_tags(&sprite_schema, load_context);
        //add tagged animation keyframes
        let frame_handles = self.load_indices(&sprite_schema, load_context);
        let sheet_handle = load_context.add_labeled_asset(KEY_SPRITE_SHEET, sprite_schema);
        Ok(SpriteSheetAsset {
            schema: sheet_handle,
            tags: tag_handles,
            frames: frame_handles,
            layout: handle,
        })
    }

    fn extensions(&self) -> &[&str] {
        &EXTENSIONS
    }
}

// endregion
