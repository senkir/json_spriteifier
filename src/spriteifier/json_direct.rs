use std::marker::PhantomData;
use bevy::asset::AssetApp;
use bevy::prelude::{App, Plugin, Asset, Bundle, Reflect, TypePath};
use bevy_common_assets::json::{JsonAssetLoader, JsonAssetPlugin};
use serde::Deserialize;
use crate::spriteifier::sprite_loader::{SpriteJsonLoader};
use crate::spriteifier::schema::SpriteSchema;

pub struct JsonSpriteifierPlugin;

impl Plugin for JsonSpriteifierPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<SpriteSchema>();
        app.register_asset_loader(
            SpriteJsonLoader
        );
    }
}

#[derive(Asset, TypePath, Deserialize, Debug, Clone, PartialEq)]
pub struct Json {
    pub data: Option<String>,

    pub sprite_schema: Option<SpriteSchema>

}