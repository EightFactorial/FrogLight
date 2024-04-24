use std::sync::Arc;

use bevy_asset::AssetServer;
use bevy_derive::Deref;
use bevy_ecs::system::{Res, Resource};

mod inner;
use inner::AssetManagerInner;

pub(crate) mod blockmap;
pub(crate) mod soundmap;

#[doc(hidden)]
pub(super) fn build(app: &mut bevy_app::App) { app.init_resource::<AssetManager>(); }

/// A manager for all assets loaded from
/// [`ResourcePacks`](crate::assets::resourcepack::ResourcePack).
#[derive(Debug, Default, Clone, Deref, Resource)]
pub struct AssetManager(Arc<AssetManagerInner>);

impl AssetManager {
    /// Returns `true` if all
    /// [`ResourcePacks`](crate::assets::resourcepack::ResourcePack)
    /// are loaded.
    #[must_use]
    pub fn all_loaded(manager: Res<Self>, assets: Res<AssetServer>) -> bool {
        manager.resourcepacks.read().iter().all(|id| assets.is_loaded_with_dependencies(id))
    }
}
