use bevy::DefaultPlugins;
use bevy::math::Vec3;
use bevy::prelude::{default, App, AssetPlugin, AssetServer, Assets, Camera2d, Commands, CommandsSceneExt, Handle, Image, IntoScheduleConfigs, Local, PluginGroup, Res, Resource, Startup, Template, Transform, Update};
use bevy::scene::{bsn, on_add_scene_patch_instance};
use json_spriteifier::animator::SpriteTimer;
use json_spriteifier::{JsonSpriteifierPlugin, SpriteSheetAsset, animator, sprite, spriteifier};
use log::info;

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
        .add_systems(Update, (report_loaded, animator::animate_sprite))
        .add_systems(Update, spawn_sprite)
        .run();
}

fn camera(mut commands: Commands) {
    commands.spawn(Camera2d::default());
}

//noinspection DuplicatedCode
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
    assets: Res<Assets<SpriteSheetAsset>>,
    crow: Res<CrowSprite>,
    mut loaded: Local<bool>,
) {
    if !*loaded {
        let texture = crow.texture.clone();
        let sheet = assets.get(&texture);
        if sheet.is_none() {
            return;
        }
        let anim = sheet.unwrap().get_animation_tag("Blink");
        let image = crow.image.clone();
        let anim_scene = spriteifier::sprite_animation(texture.clone(), "Blink");
        let timer = spriteifier::sheet_timer();
        commands.spawn_scene(
            bsn! {
            #scene
            Transform::from_xyz(1.0, 1.0, 1.0).with_scale(Vec3::splat(2.0))
            ~anim_scene
            ~spriteifier::sprite_sheet(texture.clone(), image)
            ~timer
            });

        *loaded = true;
    }
}
