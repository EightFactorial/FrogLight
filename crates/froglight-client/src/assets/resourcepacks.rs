use bevy::prelude::*;
use froglight_assets::assets::ResourcePack;
use froglight_settings::ConfigFile;
use serde::{Deserialize, Serialize};

use super::AssetLoading;

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    // Register the ResourcePackSettings and ResourcePackItem types
    app.register_type::<ResourcePackSettings>().register_type::<ResourcePackItem>();

    // Add the ResourcePackSettings ConfigFile implementation
    ResourcePackSettings::build(app);

    app.add_systems(
        Update,
        ResourcePackSettings::load_resourcepacks_on_startup
            .run_if(run_once())
            .run_if(resource_exists::<ResourcePackSettings>)
            .in_set(AssetLoading::Waiting),
    );
}

/// Settings for loading resource packs.
#[derive(Debug, Default, Clone, PartialEq, Eq, Resource, Serialize, Deserialize, Reflect)]
#[reflect(Default, Serialize, Deserialize, Resource)]
pub struct ResourcePackSettings {
    /// A list of resource packs to load.
    #[serde(default, rename = "resourcepack")]
    pub resourcepacks: Vec<ResourcePackItem>,
}

impl ResourcePackSettings {
    /// Load all [`ResourcePack`]s on startup.
    ///
    /// This system will only run once
    pub fn load_resourcepacks_on_startup(mut settings: ResMut<Self>, assets: Res<AssetServer>) {
        for item in &mut settings.resourcepacks {
            item.handle = Some(assets.load(item.path.to_string()));
        }
    }
}

/// A resource pack to load.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Reflect)]
#[reflect(Serialize, Deserialize)]
pub struct ResourcePackItem {
    /// The path to the resource pack.
    pub path: String,
    #[serde(skip)]
    #[reflect(ignore)]
    pub handle: Option<Handle<ResourcePack>>,
}

impl ConfigFile for ResourcePackSettings {
    const PATH: &'static str = "resourcepacks.toml";
}
