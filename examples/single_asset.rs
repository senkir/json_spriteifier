use bevy::DefaultPlugins;
use bevy::prelude::{
    App, AssetServer, Assets, Camera2d, Commands, Handle, Image, Local, Res, Resource, Startup,
    Update,
};
use json_spriteifier::{JsonSpriteifierPlugin, SpriteSheetAsset};
use log::info;

#[derive(Resource)]
pub struct CrowSprite {
    pub texture: Handle<SpriteSheetAsset>,
    pub image: Handle<Image>,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(JsonSpriteifierPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, report_loaded)
        .run();
}

fn setup(mut commands: Commands, assets: Res<AssetServer>) {
    commands.spawn(Camera2d);

    let texture = assets.load::<SpriteSheetAsset>("crow.json");
    let image = assets.load::<Image>("crow.png");
    commands.insert_resource(CrowSprite { texture, image });
}

fn report_loaded(
    sheet: Res<CrowSprite>,
    sheets: Res<Assets<SpriteSheetAsset>>,
    mut reported: Local<bool>,
) {
    if !*reported && sheets.get(&sheet.texture).is_some() {
        info!("crow.json loaded");
        *reported = true;
    }
}
