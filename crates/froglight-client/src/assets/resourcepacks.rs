use bevy::{asset::RecursiveDependencyLoadState, prelude::*};
use froglight_assets::assets::ResourcePack;
use froglight_settings::ConfigFile;
use serde::{Deserialize, Serialize};

use super::{AssetLoading, AssetManager, LanguageManager, SoundManager};

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    // Register the ResourcePackSettings and ResourcePackItem types
    app.register_type::<ResourcePackSettings>().register_type::<ResourcePackItem>();

    // Add the ResourcePackSettings ConfigFile implementation
    ResourcePackSettings::build(app);

    // Load all resource packs on startup
    // Only attemps to load once.
    app.add_systems(
        Update,
        ResourcePackSettings::load_resourcepacks_on_startup
            .run_if(run_once())
            .run_if(resource_exists::<ResourcePackSettings>)
            .in_set(AssetLoading::Waiting),
    );

    // Wait for all resource packs to load
    // Inserts all assets into their respective managers
    app.add_systems(
        Update,
        ResourcePackSettings::collect_resourcepack_assets
            .run_if(resource_exists::<ResourcePackSettings>)
            .run_if(resource_exists::<AssetManager>)
            .run_if(resource_exists::<LanguageManager>)
            .run_if(resource_exists::<SoundManager>)
            .in_set(AssetLoading::Loading),
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
        info!("Loading \"{}\" ResourcePacks", settings.resourcepacks.len());
        for item in &mut settings.resourcepacks {
            debug!("Loading ResourcePack: \"{}\"", item.path);
            item.handle = Some(assets.load(item.path.to_string()));
        }
    }

    /// Wait for all [`ResourcePack`]s to load.
    ///
    /// Enters the [`AssetLoading::Processing`] [`State`] when all
    /// [`ResourcePack`]s have either loaded or failed to load.
    pub fn collect_resourcepack_assets(
        asset_server: Res<AssetServer>,
        mut resourcepacks: ResMut<Assets<ResourcePack>>,

        mut asset_manager: ResMut<AssetManager>,
        mut lang_manager: ResMut<LanguageManager>,
        mut _sound_manager: ResMut<SoundManager>,

        mut settings: ResMut<ResourcePackSettings>,
        mut state: ResMut<NextState<AssetLoading>>,
    ) {
        // Check if all `ResourcePack`s either loaded or failed to load
        if settings.resourcepacks.iter().all(|pack| {
            if let Some(handle) = &pack.handle {
                matches!(
                    asset_server.get_recursive_dependency_load_state(handle),
                    Some(
                        RecursiveDependencyLoadState::Loaded | RecursiveDependencyLoadState::Failed
                    )
                )
            } else {
                true
            }
        }) {
            // Clear all assets
            asset_manager.clear();
            lang_manager.clear();
            // sound_manager.clear();

            // Take all the handles from `ResourcePackSettings`
            // and replace them with weak handles.
            {
                let mut strong_handles = Vec::with_capacity(settings.resourcepacks.len());
                for pack in &mut settings.resourcepacks {
                    if let Some(settings_handle) = &mut pack.handle {
                        strong_handles
                            .push(std::mem::replace(settings_handle, settings_handle.clone_weak()));
                    }
                }
                asset_manager.resourcepacks = strong_handles;
            }

            // Insert all assets into their respective manager.
            // This will drop all assets that went unused.
            {
                let handles: Vec<Handle<ResourcePack>> = asset_manager.resourcepacks.clone();
                for handle in handles {
                    if let Some(resourcepack) = resourcepacks.get_mut(handle) {
                        asset_manager.insert(resourcepack);
                        // Takes care of managing language strings
                        lang_manager.insert(resourcepack);
                        // Takes care of managing sounds definitions
                        // sound_manager.insert(resourcepack);
                    }
                }
            }

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
    pub path: String,
    #[serde(skip)]
    #[reflect(ignore)]
    pub handle: Option<Handle<ResourcePack>>,
}

impl ConfigFile for ResourcePackSettings {
    const PATH: &'static str = "resourcepacks.toml";
}
