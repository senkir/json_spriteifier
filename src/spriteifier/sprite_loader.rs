use crate::spriteifier::schema::SpriteSchema;
use bevy::asset::io::Reader;
use bevy::asset::{Asset, AssetLoader, LoadContext, LoadedAsset};
use bevy::math::UVec2;
use bevy::prelude::{Component, TypePath};
use bevy::tasks::ConditionalSendFuture;
use serde_json::from_slice;

/// Loader capable of handling sprite sheet json data
#[derive(Default, TypePath)]
pub struct SpriteJsonLoader;

impl SpriteJsonLoader {
    pub const SUPPORTED_FORMATS: [&'static str; 1] = ["json"];
    pub fn new() -> SpriteJsonLoader {
        SpriteJsonLoader {
            
        }
    }
}

impl SpriteJsonLoader {

    fn load_frames(
        &self,
        load_context: &mut LoadContext,
        sprites: &SpriteSchema,
    ) {
        for frame in sprites.frames.iter() {
            if (!frame.filename.is_some()) {
                //TODO: mark error in parsing
                continue;
            }
            let name = frame.filename.unwrap();
            let frame = frame.to_sprite_frame();
            load_context.add_loaded_labeled_asset(name, frame); //TODO: is this worth it
        }
    }
    

    fn load_tile_assets(&self) {
        
    }
}

impl AssetLoader for SpriteJsonLoader {
    type Asset = SpriteSchema;

    type Settings = ();
    type Error = Box<dyn std::error::Error + Send + Sync>;

    async fn load(
        &self,
        reader: &mut dyn Reader,
        settings: &Self::Settings,
        load_context: &mut LoadContext<'_>,
    ) -> Result<SpriteSchema, Self::Error> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;
        let asset = from_slice::<SpriteSchema>(&bytes)?;
        Ok(asset)    
    }

    fn extensions(&self) -> &[&str] {
        &["json", "sprites.json", "tiles.json"]
    }
}
