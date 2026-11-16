use crate::spriteifier::schema::SpriteSchema;
use bevy::asset::AssetApp;
use bevy::prelude::{App, Asset, Plugin, TypePath};
use bevy_common_assets::json::JsonAssetPlugin;
use serde::Deserialize;

pub struct JsonSpritePlugin;

impl Plugin for JsonSpritePlugin {
    fn build(&self, app: &mut App) {
        // load custom schema and JsonAssetPlugin for it
        app.init_asset::<SpriteSchema>();
        app.add_plugins(JsonAssetPlugin::<SpriteSchema>::new(&["json"]));
    }
}