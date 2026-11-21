use bevy::prelude::{Asset, TypePath};

/// representation of a recorded sprite frame within an overall sheet as an asset.
#[derive(Asset, Clone, TypePath)]
pub struct SpriteFrameAsset {
    pub rotated: bool,
    pub duration: usize,
    pub index: usize
}