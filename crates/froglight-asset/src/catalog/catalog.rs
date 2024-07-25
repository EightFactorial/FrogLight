use std::{any::TypeId, hash::BuildHasherDefault};

use bevy_app::App;
use bevy_asset::{Asset, AssetId, Assets, Handle, UntypedAssetId};
use bevy_derive::{Deref, DerefMut};
use bevy_ecs::{reflect::ReflectResource, system::Resource};
use bevy_reflect::{std_traits::ReflectDefault, Reflect};
use bevy_utils::{AHasher, Entry, HashMap, TypeIdMap};
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
    pub(super) inner: TypeIdMap<UntypedAssetMap>,
}

/// A map of [`ResourceKey`]s to [`UntypedAssetId`]s.
#[derive(Debug, Default, Clone, Deref, DerefMut, Reflect)]
pub(super) struct UntypedAssetMap(#[reflect(ignore)] HashMap<ResourceKey, UntypedAssetId>);

impl AssetCatalog {
    /// Creates a new [`AssetCatalog`].
    ///
    /// Equivalent to [`AssetCatalog::default()`].
    #[must_use]
    pub fn new() -> Self { Self::default() }

    /// Inserts an asset into the catalog.
    ///
    /// Returns the old asset id if one existed.
    pub fn insert<A: Asset>(
        &mut self,
        key: ResourceKey,
        id: impl Into<AssetId<A>>,
    ) -> Option<AssetId<A>> {
        let map = self.inner.entry(TypeId::of::<A>()).or_default();
        let old = map.insert(key, Into::<AssetId<A>>::into(id).untyped());
        old.map(UntypedAssetId::typed_debug_checked)
    }

    /// Gets an asset from the catalog by it's key.
    #[must_use]
    pub fn get<A: Asset>(&self, key: &str) -> Option<AssetId<A>> {
        let map = self.inner.get(&TypeId::of::<A>())?;
        map.get(key).copied().map(UntypedAssetId::typed_debug_checked)
    }

    /// Creates a new [`Handle`] to the asset with the given key.
    #[must_use]
    pub fn create_handle<A: Asset>(&self, key: &str, assets: &mut Assets<A>) -> Option<Handle<A>> {
        self.get::<A>(key).and_then(|id| assets.get_strong_handle(id))
    }

    /// Gets an [`Entry`] into the catalog for the given key.
    ///
    /// # Note
    /// Be very careful and make sure the [`UntypedAssetId`]
    /// is the correct type!
    #[must_use]
    pub fn entry<A: Asset>(
        &mut self,
        key: ResourceKey,
    ) -> Entry<ResourceKey, UntypedAssetId, BuildHasherDefault<AHasher>> {
        self.inner.entry(TypeId::of::<A>()).or_default().entry(key)
    }

    /// Removes an asset from the catalog by it's key.
    pub fn remove<A: Asset>(&mut self, key: &str) -> Option<AssetId<A>> {
        let map = self.inner.get_mut(&TypeId::of::<A>())?;
        map.remove(key).map(UntypedAssetId::typed_debug_checked)
    }

    /// Returns the number of assets of type `A` in the catalog.
    #[must_use]
    pub fn len_of<A: Asset>(&self) -> usize {
        self.inner.get(&TypeId::of::<A>()).map_or(0, |map| map.len())
    }

    /// Clears the catalog, removing all assets.
    ///
    /// Keeps the allocated memory for future use.
    pub fn clear(&mut self) {
        for map in self.inner.values_mut() {
            map.clear();
        }
    }
}
