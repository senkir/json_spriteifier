use crate::animator::SpriteAnimationAsset;
use crate::frame::SpriteFrameAsset;
use crate::parser::schema::SpriteSheet;
use crate::sheet::{KEY_TEXTURE_LAYOUT, SpriteSheetAsset};
use bevy::asset::io::Reader;
use bevy::asset::{AssetLoader, LoadContext};
use bevy::image::TextureAtlasLayout;
use bevy::prelude::{info, TypePath};
use serde_json::from_slice;
use std::collections::HashMap;

/// supported extensions for the loader
const EXTENSIONS: [&str; 1] = ["json"];

// region loader

/// Loader capable of handling sprite sheet json data
#[derive(Default, TypePath)]
pub struct SpriteJsonLoader;

impl SpriteJsonLoader {
    pub const SUPPORTED_FORMATS: [&'static str; 1] = ["json"];
    pub fn new() -> SpriteJsonLoader {
        SpriteJsonLoader {}
    }

    fn load_asset_layout(sprite_schema: &SpriteSheet) -> TextureAtlasLayout {
        let mut layout = TextureAtlasLayout::new_empty(sprite_schema.sheet_size());
        for frame in sprite_schema.frames.iter() {
            layout.add_texture(frame.frame);
        }
        layout
    }

    fn load_animation_tags(sprites: &SpriteSheet) -> HashMap<String, SpriteAnimationAsset> {
        let mut out = HashMap::with_capacity(sprites.meta.frame_tags.len());
        for tag in sprites.meta.frame_tags.iter() {
            let anim_asset = SpriteAnimationAsset::from(tag.clone());
            out.insert(anim_asset.tag.clone(), anim_asset.clone());
        }
        out
    }

    fn load_indices(sprites: &SpriteSheet) -> HashMap<String, SpriteFrameAsset> {
        let mut out = HashMap::with_capacity(sprites.frames.len());
        for (i, frame) in sprites.frames.iter().enumerate() {
            if frame.name.is_none() {
                continue;
            }
            let name = frame.name.clone().unwrap();
            let asset = SpriteFrameAsset {
                rotated: frame.rotated,
                duration: frame.duration,
                index: i,
            };
            out.insert(name, asset);
        }
        out
    }

    fn load_sprite_frames_array(sprites: &SpriteSheet) -> Vec<SpriteFrameAsset> {
        let mut out = Vec::with_capacity(sprites.frames.len());
        for (i, frame) in sprites.frames.iter().enumerate() {
            if frame.name.is_none() {
                continue;
            }
            let asset = SpriteFrameAsset {
                rotated: frame.rotated,
                duration: frame.duration,
                index: i,
            };
            out.push(asset);
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

        let atlas_layout = SpriteJsonLoader::load_asset_layout(&sprite_schema);
        let atlas_layout_handle = load_context.add_labeled_asset(KEY_TEXTURE_LAYOUT, atlas_layout);

        let tag_handles = SpriteJsonLoader::load_animation_tags(&sprite_schema);

        //add tagged animation keyframes
        let frame_handles = SpriteJsonLoader::load_sprite_frames_array(&sprite_schema);
        Ok(SpriteSheetAsset::new(
            &sprite_schema,
            &atlas_layout_handle,
            tag_handles,
            frame_handles,
        ))
    }

    fn extensions(&self) -> &[&str] {
        &EXTENSIONS
    }
}

// endregion

#[cfg(test)]
mod test {
    use super::*;
    use crate::SpriteAtFrame;
    use bevy::utils::default;

    fn vec_frames(size: usize) -> Vec<SpriteAtFrame> {
        let mut frames = Vec::with_capacity(size);
        for _ in 0..size {
            frames.push(SpriteAtFrame {
                duration: size,
                name: Some(format!("frame{}", size)),
                ..default()
            });
        }
        frames
    }

    #[test]
    fn test_load_asset_layout() {
        let frames = vec_frames(3);
        let sheet_in = SpriteSheet {
            frames,
            ..default()
        };
        let layout = SpriteJsonLoader::load_asset_layout(&sheet_in);
        assert_eq!(layout.textures.len(), 3)
    }
}
