use bevy::{asset::RecursiveDependencyLoadState, prelude::*};
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

    // Load all resource packs on startup
    // Only attempts to load once.
    app.add_systems(
        Update,
        ResourcePackSettings::load_resourcepacks_on_startup
            .run_if(run_once())
            .run_if(resource_exists::<ResourcePackSettings>)
            .in_set(AssetLoading::Waiting),
    );

    // Waits for all `ResourcePack`s to load or fail
    app.add_systems(
        Update,
        ResourcePackSettings::wait_for_resourcepacks
            .run_if(resource_exists::<ResourcePackSettings>)
            .in_set(AssetLoading::Loading),
    );
}

/// Settings for loading resource packs.
///
/// # Note
/// Every frame this is accessed mutably it will save the settings to disk.
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
    /// This system will only run once, if the client is in
    /// the [`AssetLoading::Waiting`] [`State`] the first time
    /// the [`Update`] [`Schedule`] is run.
    ///
    /// Enters the [`AssetLoading::Loading`] [`State`] after running.
    pub fn load_resourcepacks_on_startup(
        assets: Res<AssetServer>,
        mut settings: ResMut<Self>,
        mut state: ResMut<NextState<AssetLoading>>,
    ) {
        // Enter the loading state
        state.set(AssetLoading::Loading);
        debug!("Entering AssetLoading::Loading");

        // Load all resource packs
        info!("Loading {} ResourcePack(s)", settings.resourcepacks.len());
        for item in &mut settings.resourcepacks {
            if let Some(path) = &item.path {
                debug!("Loading: \"{}\"", path);
                item.handle = Some(assets.load(path.to_string()));
            } else if item.handle.is_none() {
                error!("`ResourcePackItem` has no path or handle?");
            }
        }
    }

    /// Wait for all [`ResourcePack`]s to load.
    ///
    /// Enters the [`AssetLoading::Processing`] [`State`] when all
    /// [`ResourcePack`]s have either loaded or failed to load.
    pub fn wait_for_resourcepacks(
        assets: Res<AssetServer>,
        settings: Res<ResourcePackSettings>,
        mut state: ResMut<NextState<AssetLoading>>,
    ) {
        // Check if all `ResourcePack`s either loaded or failed to load
        if settings.resourcepacks.iter().all(|pack| {
            if let Some(handle) = &pack.handle {
                matches!(
                    assets.get_recursive_dependency_load_state(handle),
                    Some(
                        RecursiveDependencyLoadState::Loaded | RecursiveDependencyLoadState::Failed
                    )
                )
            } else {
                true
            }
        }) {
            // Enter the processing state, as all assets have loaded
            debug!("Entering AssetLoading::Processing");
            state.set(AssetLoading::Processing);
        }
    }
}

/// A resource pack to load.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Reflect)]
#[reflect(Serialize, Deserialize)]
pub struct ResourcePackItem {
    /// The path to the resource pack.
    ///
    /// Only [`ResourcePack`]s with a path will be saved/loaded from disk.
    pub path: Option<String>,
    #[serde(skip)]
    #[reflect(ignore)]
    pub handle: Option<Handle<ResourcePack>>,
}

impl ConfigFile for ResourcePackSettings {
    const PATH: &'static str = "resourcepacks.toml";

    fn deserialize_map(mut self) -> Self {
        // Remove any `ResourcePackItem`s without a path
        self.resourcepacks.retain(|item| item.path.is_some());
        self
    }

    fn serialize_map(mut self) -> Self {
        // Remove any `ResourcePackItem`s without a path
        self.resourcepacks.retain(|item| item.path.is_some());
        self
    }
}
