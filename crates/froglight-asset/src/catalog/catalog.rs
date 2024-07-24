use std::any::TypeId;

use bevy_app::App;
use bevy_asset::{Asset, AssetId, UntypedAssetId};
use bevy_derive::{Deref, DerefMut};
use bevy_ecs::{reflect::ReflectResource, system::Resource};
use bevy_reflect::{std_traits::ReflectDefault, Reflect};
use bevy_utils::{HashMap, TypeIdMap};
use froglight_common::ResourceKey;

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    app.register_type::<AssetCatalog>().init_resource::<AssetCatalog>();
}

/// A catalog of assets.
///
/// Allows associating asset keys with asset ids.
#[derive(Debug, Default, Resource, Reflect)]
#[reflect(Default, Resource)]
pub struct AssetCatalog {
    pub(super) storage: TypeIdMap<UntypedAssetMap>,
}

/// A map of [`ResourceKey`]s to [`UntypedAssetId`]s.
#[derive(Debug, Default, Clone, Deref, DerefMut, Reflect)]
pub(super) struct UntypedAssetMap(#[reflect(ignore)] HashMap<ResourceKey, UntypedAssetId>);

impl AssetCatalog {
    /// Creates a new [`AssetStorage`].
    ///
    /// Equivalent to [`AssetStorage::default()`].
    #[must_use]
    pub fn new() -> Self { Self::default() }

    /// Inserts an asset into the storage.
    ///
    /// Returns the old asset ID if one existed.
    pub fn insert<A: Asset>(
        &mut self,
        key: ResourceKey,
        id: impl Into<AssetId<A>>,
    ) -> Option<AssetId<A>> {
        let map = self.storage.entry(TypeId::of::<A>()).or_default();
        let old = map.insert(key, Into::<AssetId<A>>::into(id).untyped());
        old.map(UntypedAssetId::typed_debug_checked)
    }

    /// Gets an asset from the storage by it's key.
    #[must_use]
    pub fn get<A: Asset>(&self, key: &str) -> Option<AssetId<A>> {
        let map = self.storage.get(&TypeId::of::<A>())?;
        map.get(key).copied().map(UntypedAssetId::typed_debug_checked)
    }

    /// Removes an asset from the storage by it's key.
    pub fn remove<A: Asset>(&mut self, key: &str) -> Option<AssetId<A>> {
        let map = self.storage.get_mut(&TypeId::of::<A>())?;
        map.remove(key).map(UntypedAssetId::typed_debug_checked)
    }

    /// Clears the storage, removing all assets.
    ///
    /// Keeps the allocated memory for future use.
    pub fn clear(&mut self) {
        for map in self.storage.values_mut() {
            map.clear();
        }
    }
}
