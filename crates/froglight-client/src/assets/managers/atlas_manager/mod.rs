use std::ops::Not;

use bevy::{asset::RecursiveDependencyLoadState, prelude::*};
use froglight_network::common::ResourceKey;
use hashbrown::HashMap;

use super::AssetManager;
use crate::assets::AssetLoading;

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    app.init_resource::<AtlasManager>().register_type::<AtlasManager>();

    app.add_systems(
        OnEnter(AssetLoading::Loading),
        AtlasManager::reset_atlas_manager.run_if(resource_exists::<AtlasManager>),
    );
    app.add_systems(
        Update,
        AtlasManager::populate_atlas_manager
            .run_if(not(AtlasManager::is_finished))
            .run_if(resource_exists::<AtlasManager>)
            .run_if(AssetManager::is_finished)
            .run_if(resource_exists::<AssetManager>)
            .after(AssetManager::populate_asset_manager)
            .in_set(AssetLoading::Processing),
    );
}

/// A [`Resource`] for managing [`TextureAtlasLayout`] assets.
#[derive(Debug, Default, Clone, Resource, Reflect)]
pub struct AtlasManager {
    /// Texture Atlases
    pub atlases: HashMap<ResourceKey, Handle<TextureAtlasLayout>>,
}

impl AtlasManager {
    /// Returns `true` if the [`AtlasManager`] has finished loading all assets.
    #[must_use]
    pub fn is_finished(res: Res<AtlasManager>, assets: Res<AssetServer>) -> bool {
        res.atlases.is_empty().not()
            && res.atlases.iter().all(|(_, handle)| {
                matches!(
                    assets.get_recursive_dependency_load_state(handle),
                    Some(
                        RecursiveDependencyLoadState::Loaded | RecursiveDependencyLoadState::Failed
                    )
                )
            })
    }

    /// Resets the [`AtlasManager`] to its initial state.
    fn reset_atlas_manager(mut res: ResMut<AtlasManager>) { res.atlases.clear(); }

    /// Populates the [`AtlasManager`] with blocks from currently loaded assets.
    ///
    /// Relies on the [`AssetManager`] to have finished loading.
    pub(crate) fn populate_atlas_manager(
        _image_assets: ResMut<Assets<Image>>,
        _atlas_assets: ResMut<Assets<TextureAtlasLayout>>,

        _asset_manager: Res<AssetManager>,
        _atlas_manager: ResMut<AtlasManager>,
    ) {
    }
}
