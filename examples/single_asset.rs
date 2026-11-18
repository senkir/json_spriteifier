use bevy::DefaultPlugins;
use bevy::prelude::{App, AssetServer, Assets, Camera2d, Commands, Res, Resource, Startup, Update};
use json_spriteifier::{JsonSpriteifierPlugin, SpriteSheetAsset};

#[derive(Resource)]
pub struct CrowSprite {
    pub texture: Handle<SpriteSheetAsset>,
    pub image: Handle<Image>,
}


#[derive(Resource)]
struct CrowSheet(Handle<SpriteSheetHandle>);

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
    commands.insert_resource(CrowSheet(assets.load::<SpriteSheetAsset>("crow.json")));
}

fn report_loaded(
    sheet: Res<CrowSheet>,
    sheets: Res<Assets<SpriteSheetHandle>>,
    mut reported: Local<bool>,
) {
    if !*reported && sheets.get(&sheet.0).is_some() {
        info!("crow.json loaded");
        *reported = true;
    }
}