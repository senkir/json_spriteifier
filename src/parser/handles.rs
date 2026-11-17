use crate::parser::schema::SpriteSheet;
use bevy::asset::Assets;
use bevy::image::TextureAtlasLayout;
use bevy::prelude::{AssetServer, Commands, Handle, Res, Resource};

/// load an asset by image and json path into a `TextureAtlasLayout`
/// asset loading is asynchronous so loading will trigger it to be handled by processor later
// pub fn load_asset(
//     image_path: &str,
//     json_path: &str,
//     asset_server: &AssetServer,
//     texture_atlas_layouts: Res<Assets<TextureAtlasLayout>>,
// ) {
//     let texture: Handle<Image> = asset_server.load(image_path);
// }

/// handle to a sprite schema loaded from disc
#[derive(Resource)]
pub struct SchemaHandle(Handle<SpriteSheet>);

// this is an example implementation fo setup that loads a single json.
pub fn setup(asset_server: Res<AssetServer>, mut commands: Commands) {
    //TODO: the sprite path should load here
    let handle: Handle<SpriteSheet> = asset_server.load("sprites.json");
    commands.insert_resource(SchemaHandle(handle));
}

/// handle the condition where schema assets are ready
pub fn on_asset_ready(
    mut commands: Commands,
    handle: Res<SchemaHandle>,
    schemas: Res<Assets<SpriteSheet>>,
) {
    if let Some(schema) = schemas.get(&handle.0) {
        //do stuff with the sprite sheet
    };
}

pub fn handle_json_sprite_sheet(
    sprite_schema: &SpriteSheet,
    texture_atlas_layouts: &mut Assets<TextureAtlasLayout>,
) {
    //TODO: maybe this async

    let columns = sprite_schema.col_count();
    let rows = sprite_schema.row_count();

    let layout =
        TextureAtlasLayout::from_grid(sprite_schema.sprite_size(), columns, rows, None, None);

    texture_atlas_layouts.add(layout);
}
