//! A brief overview of what your library does.
//!
//! # Examples
//! ```
//! // Usage example here
//! ```
//!

pub mod animator;
pub mod parser;
pub mod frame;
pub mod loader;
pub mod sheet;
pub mod sprite;

use bevy::app::{App, Plugin};
use bevy::prelude::AssetApp;

pub use parser::schema::{SpriteSheet, SpriteAtFrame};
pub use animator::{SpriteAnimationAsset};
pub use frame::SpriteFrameAsset;
pub use loader::{SpriteJsonLoader};
pub use sheet::{SpriteSheetAssetTemplate, SpriteSheetAsset};
pub use sprite::spriteifier;

/// The primary plugin
pub struct JsonSpriteifierPlugin;

impl Plugin for JsonSpriteifierPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<SpriteSheet>();
        app.init_asset::<SpriteSheetAsset>();
        app.init_asset::<SpriteFrameAsset>();
        app.init_asset::<SpriteAnimationAsset>();
        app.register_asset_loader(SpriteJsonLoader);
    }
}
