use bevy::DefaultPlugins;
use bevy::math::Vec3;
use bevy::prelude::{
    App, AssetPlugin, AssetServer, Assets, Camera2d, Commands, CommandsSceneExt, Handle, Image,
    IntoScheduleConfigs, Local, PluginGroup, Res, Resource, Startup, Transform, Update, default,
};
use bevy::scene::bsn;
use json_spriteifier::sheet::{SpriteSheetAsset, SpriteSheetAssetTemplate};
use json_spriteifier::{JsonSpriteifierPlugin};
use log::info;

const ANIMATION_TAG: &'static str = "Blink";

#[derive(Resource)]
pub struct CrowSprite {
    pub texture: Handle<SpriteSheetAsset>,
    pub image: Handle<Image>,
}

fn main() {
    App::new()
        //point to example assets directory
        .add_plugins(DefaultPlugins.set(AssetPlugin {
            file_path: "examples/assets/".into(),
            ..default()
        }))
        .add_plugins(JsonSpriteifierPlugin)
        .add_systems(Startup, (camera, setup).chain())
        .add_systems(Update, report_loaded)
        .add_systems(Update, spawn_sprite)
        .run();
}

fn camera(mut commands: Commands) {
    commands.spawn(Camera2d::default());
}

fn setup(mut commands: Commands, assets: Res<AssetServer>) {
    let texture = assets.load::<SpriteSheetAsset>("crow.json");
    let image = assets.load::<Image>("crow.png");
    commands.insert_resource(CrowSprite { texture, image });
}

//noinspection DuplicatedCode
fn report_loaded(
    crow: Res<CrowSprite>,
    sheets: Res<Assets<SpriteSheetAsset>>,
    mut reported: Local<bool>,
    mut reported_img: Local<bool>,
) {
    if !*reported && sheets.get(&crow.texture).is_some() {
        info!("crow.json loaded");
        *reported = true;
    }

    if !*reported_img && sheets.get(&crow.texture).is_some() {
        info!("crow.png loaded");
        *reported_img = true;
    }
}

fn spawn_sprite(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    crow: Res<CrowSprite>,
    mut loaded: Local<bool>,
) {
    if !*loaded {
        let texture = crow.texture.clone();
        let image = crow.image.clone();
        commands.spawn_scene(bsn! {
            #scene
            Transform::from_xyz(1.0, 1.0, 1.0).with_scale(Vec3::splat(2.0))
            ~SpriteSheetAssetTemplate::from(texture)
            .with_image(image)
            .with_index(7)
        });

        *loaded = true;
    }
}
