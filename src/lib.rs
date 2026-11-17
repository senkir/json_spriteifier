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

use bevy::app::{App, Plugin};
use bevy::prelude::AssetApp;

pub use parser::schema::{SpriteSheet, SpriteAtFrame};
pub use animator::{SpriteAnimationAsset, SpriteAnimationMode};
pub use frame::SpriteFrameAsset;
use crate::loader::{SpriteJsonLoader, SpriteSheetAsset};

/// The primary plugin
pub struct JsonSpriteifierPlugin;

impl Plugin for JsonSpriteifierPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<SpriteSheetAsset>();
        app.init_asset::<SpriteFrameAsset>();
        app.init_asset::<SpriteAnimationAsset>();
        app.register_asset_loader(SpriteJsonLoader);
    }
}
