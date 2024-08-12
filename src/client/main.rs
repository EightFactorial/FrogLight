#![doc = include_str!("../../README.md")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

use bevy::{
    app::{App, AppExit, Update},
    asset::{AssetServer, Assets},
    math::Vec3,
    pbr::{
        AmbientLight, DirectionalLight, DirectionalLightBundle, PbrBundle, StandardMaterial,
        UvChannel,
    },
    prelude::{
        run_once, Commands, IntoSystemConfigs, Mesh, OnEnter, Res, ResMut, Transform, Visibility,
    },
};
use froglight::{
    asset::{
        assets::processed::ResourceAtlas, AssetCatalog, AssetKey, AssetState, ResourceLoadTrigger,
        ResourcePackList,
    },
    prelude::ResourceKey,
    ApplicationPlugins,
};
use mimalloc::MiMalloc;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

#[cfg(feature = "inspector")]
mod inspector;

fn main() -> AppExit {
    let mut app = App::new();
    app.add_plugins(ApplicationPlugins);

    #[cfg(feature = "inspector")]
    {
        app.add_plugins(inspector::InspectorPlugin);
    }

    app.add_systems(Update, load_minecraft_zip.run_if(run_once()));
    app.add_systems(OnEnter(AssetState::Loaded), spawn_models.run_if(run_once()));

    app.run()
}

fn load_minecraft_zip(
    assets: Res<AssetServer>,
    mut list: ResMut<ResourcePackList>,
    mut commands: Commands,
) {
    list.push(assets.load("frog://resourcepacks/minecraft.zip"));
    commands.trigger(ResourceLoadTrigger);

    commands.insert_resource(AmbientLight::default());
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight { shadows_enabled: true, ..Default::default() },
        ..Default::default()
    });
}

const ATLAS_KEY: &str = "minecraft:blocks";

const KEY_ONE: &str = "minecraft:block/yellow_candle_cake";
const KEY_TWO: &str = "minecraft:block/composter";

fn spawn_models(
    catalog: Res<AssetCatalog>,
    atlases: Res<Assets<ResourceAtlas>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut commands: Commands,
) {
    let Some(atlas) = catalog.get::<ResourceAtlas>(ATLAS_KEY).and_then(|id| atlases.get(id)) else {
        return;
    };

    let material = materials.add(StandardMaterial {
        base_color_channel: UvChannel::Uv1,
        ..StandardMaterial::from(atlas.atlas_image.clone())
    });

    commands.spawn((
        AssetKey::<Mesh>::new(ResourceKey::const_new(KEY_ONE)),
        PbrBundle {
            material: material.clone(),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, -6.0)),
            ..Default::default()
        },
    ));

    commands.spawn((
        AssetKey::<Mesh>::new(ResourceKey::const_new(KEY_TWO)),
        PbrBundle {
            material: material.clone(),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, -6.0)),
            visibility: Visibility::Hidden,
            ..Default::default()
        },
    ));
}
