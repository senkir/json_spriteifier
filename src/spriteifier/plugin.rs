use crate::spriteifier::sprite_loader::{SpriteAsset, SpriteJsonLoader};
use bevy::asset::AssetApp;
use bevy::prelude::{App, Plugin};

pub struct JsonSpriteifierPlugin;

impl Plugin for JsonSpriteifierPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<SpriteAsset>();
        app.register_asset_loader(SpriteJsonLoader);
    }
}
