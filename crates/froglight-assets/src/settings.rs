use bevy_app::{App, Update};
use bevy_asset::{AssetEvent, AssetServer};
use bevy_derive::{Deref, DerefMut};
use bevy_ecs::{
    schedule::{
        common_conditions::{in_state, on_event, state_changed},
        Condition, IntoSystemConfigs,
    },
    system::{Res, ResMut, Resource},
};
use compact_str::CompactString;
use froglight_settings::ConfigFile;
use serde::{Deserialize, Serialize};

use crate::{assets::resourcepack::ResourcePack, AssetLoadingState, AssetManager};

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    // Add loading and saving systems
    ResourcePackSettings::build(app);

    // Load a resource pack
    app.add_systems(
        Update,
        ResourcePackSettings::load_resourcepack
            .run_if(
                state_changed::<AssetLoadingState>
                    .and_then(in_state(AssetLoadingState::LoadingResources))
                    .or_else(on_event::<AssetEvent<ResourcePack>>()),
            )
            .in_set(AssetLoadingState::LoadingResources),
    );
}

/// A list of paths to the currently loaded
/// [`ResourcePacks`](crate::assets::resourcepack::ResourcePack).
#[derive(
    Debug, Default, Clone, PartialEq, Eq, Hash, Deref, DerefMut, Resource, Serialize, Deserialize,
)]
pub struct ResourcePackSettings {
    #[deref]
    #[serde(rename = "resourcepacks")]
    paths: Vec<ResourcePackItem>,
    #[serde(skip, default)]
    index: usize,
}

/// A path to a [`ResourcePack`](crate::assets::resourcepack::ResourcePack).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deref, DerefMut, Serialize, Deserialize)]
pub struct ResourcePackItem {
    /// The path
    pub path: CompactString,
}

impl ConfigFile for ResourcePackSettings {
    const PATH: &'static str = "resourcepacks.toml";
}

impl ResourcePackSettings {
    pub(crate) fn load_resourcepack(
        mut list: ResMut<Self>,
        manager: Res<AssetManager>,
        assets: Res<AssetServer>,
    ) {
        // Reset the current index if the list is empty
        if manager.resourcepacks.read().is_empty() {
            list.index = 0;
        }

        // Load the next resource pack
        if let Some(item) = list.paths.get(list.index) {
            #[cfg(debug_assertions)]
            bevy_log::debug!("Loading ResourcePack: \"{}\"", item.path);

            let handle = assets.load(item.path.to_string());
            manager.resourcepacks.write().push(handle);

            list.index += 1;
        }
    }

    /// Returns `true` if all
    /// [`ResourcePacks`](crate::assets::resourcepack::ResourcePack) are loaded.
    #[must_use]
    pub fn all_loaded(list: Res<Self>) -> bool { list.index >= list.paths.len() }
}
