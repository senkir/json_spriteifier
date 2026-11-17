//! A brief overview of what your library does.
//!
//! # Examples
//! ```
//! // Usage example here
//! ```
//!

pub mod animator;
pub mod assets;
pub mod parser;

use bevy::app::{App, Plugin};
use bevy::prelude::AssetApp;

pub use parser::schema::{SpriteSheet, SpriteSheetIndex};
pub use assets::loader::{SpriteAsset, SpriteJsonLoader};

/// The primary plugi
pub struct JsonSpriteifierPlugin;

impl Plugin for JsonSpriteifierPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<SpriteAsset>();
        app.register_asset_loader(SpriteJsonLoader);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        //TODO: fill this in maybe
        assert!(2 + 2 == 4);
    }
}
