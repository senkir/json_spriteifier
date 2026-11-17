use crate::animator::SpriteAnimationAsset;
use crate::parser::schema::{FrameTag, SpriteSheet};
use bevy::asset::io::Reader;
use bevy::asset::{Asset, AssetLoader, LoadContext};
use bevy::image::TextureAtlasLayout;
use bevy::prelude::{Handle, TypePath};
use serde_json::from_slice;

/// supported extensions for the loader
const EXTENSIONS: [&str; 2] = ["json", "sprites.json"];

/// Asset holding loaded data
#[derive(Asset, TypePath)]
pub struct SpriteAsset {
    pub schema: SpriteSheet,
    pub tags: Vec<SpriteAnimationAsset>,
    pub layout: Handle<TextureAtlasLayout>,
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

// region loader

/// Loader capable of handling sprite sheet json data
#[derive(Default, TypePath)]
pub struct SpriteJsonLoader;

impl SpriteJsonLoader {
    pub const SUPPORTED_FORMATS: [&'static str; 1] = ["json"];
    pub fn new() -> SpriteJsonLoader {
        SpriteJsonLoader {}
    }

    fn load_animation_tags(&self, sprites: &SpriteSheet) -> Vec<SpriteAnimationAsset> {
        let mut vec = Vec::new();
        for tag in sprites.meta.frame_tags.iter() {
            vec.push(SpriteAnimationAsset::from_tag(tag.clone()));
        }
        vec
    }

    fn load_asset_layout(&self, sprite_schema: &SpriteSheet) -> TextureAtlasLayout {
        let mut layout = TextureAtlasLayout::new_empty(sprite_schema.sheet_size());
        for frame in sprite_schema.frames.iter() {
            layout.add_texture(frame.rect());
        }
        layout
    }
}

impl AssetLoader for SpriteJsonLoader {
    type Asset = SpriteAsset;

    type Settings = ();
    type Error = Box<dyn std::error::Error + Send + Sync>;

    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &Self::Settings,
        load_context: &mut LoadContext<'_>,
    ) -> Result<SpriteAsset, Self::Error> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;
        let sprite_schema = from_slice::<SpriteSheet>(&bytes)?;

        let non_grid_layout = self.load_asset_layout(&sprite_schema);
        let handle = load_context.add_labeled_asset("layout", non_grid_layout);
        let tags = self.load_animation_tags(&sprite_schema);
        let asset = SpriteAsset {
            schema: sprite_schema,
            tags,
            layout: handle,
        };
        Ok(asset)
    }

    fn extensions(&self) -> &[&str] {
        &EXTENSIONS
    }
}

// endregion
