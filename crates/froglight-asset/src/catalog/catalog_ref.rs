use std::{
    hash::{BuildHasherDefault, Hash},
    marker::PhantomData,
};

use bevy_asset::{Asset, Handle, UntypedHandle};
use bevy_utils::{
    hashbrown::{hash_map::EntryRef, Equivalent},
    AHasher, Entry,
};
use froglight_common::ResourceKey;

use super::{
    catalog::UntypedAssetMap,
    catalog_iter::{Typed, Untyped},
    CatalogIter, CatalogIterMut,
};

/// A reference to an [`Asset`]'s [`AssetCatalog`](super::AssetCatalog).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypedCatalogRef<'a, A: Asset>(pub(super) &'a UntypedAssetMap, PhantomData<A>);

impl<'a, A: Asset> TypedCatalogRef<'a, A> {
    /// Creates a new [`TypedCatalogRef`] from an [`UntypedAssetMap`].
    #[must_use]
    pub(super) const fn new(map: &'a UntypedAssetMap) -> Self { Self(map, PhantomData) }

    /// Returns a [`Handle`] to an [`Asset`].
    #[must_use]
    pub fn get(&self, asset: &str) -> Option<Handle<A>> {
        Self::get_untyped(self, asset).cloned().map(UntypedHandle::typed_debug_checked)
    }

    /// Returns an [`UntypedHandle`] to an [`Asset`].
    #[must_use]
    pub fn get_untyped(&self, asset: &str) -> Option<&UntypedHandle> { self.0.get(asset) }

    /// Returns `true` if the [`TypedCatalogRef`] contains the asset.
    #[must_use]
    pub fn contains(&self, asset: &str) -> bool { self.0.contains_key(asset) }

    /// Returns the number of assets in the [`TypedCatalogRef`].
    #[must_use]
    pub fn len(&self) -> usize { self.0.len() }

    /// Returns `true` if the [`TypedCatalogRef`] is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool { self.0.is_empty() }

    /// Returns an iterator over the [`Handle`]s in the [`TypedCatalogRef`].
    ///
    /// # Note
    /// This clones every [`UntypedHandle`] and casts it into a [`Handle`].
    ///
    /// Consider using [`TypedCatalogRef::iter_untyped`] if you only need
    /// [`UntypedHandle`]s.
    #[must_use]
    pub fn iter_typed(&'a self) -> CatalogIter<'a, A, Typed> { CatalogIter::from(self) }

    /// Returns an iterator over the [`UntypedHandle`]s in the
    /// [`TypedCatalogRef`].
    #[must_use]
    pub fn iter_untyped(&'a self) -> CatalogIter<'a, A, Untyped> { CatalogIter::from(self) }
}

/// A mutable reference to an [`Asset`]'s [`AssetCatalog`](super::AssetCatalog).
#[derive(Debug, PartialEq, Eq)]
pub struct TypedCatalogMut<'a, A: Asset>(pub(super) &'a mut UntypedAssetMap, PhantomData<A>);

impl<'a, A: Asset> TypedCatalogMut<'a, A> {
    /// Creates a new [`TypedCatalogMut`] from an [`UntypedAssetMap`].
    #[must_use]
    pub(super) fn new(map: &'a mut UntypedAssetMap) -> Self { Self(map, PhantomData) }

    /// Returns a [`Handle`] to an [`Asset`].
    #[must_use]
    pub fn get(&self, asset: &str) -> Option<Handle<A>> {
        self.0.get(asset).cloned().map(UntypedHandle::typed_debug_checked)
    }

    /// Inserts a [`Handle`] into the [`TypedCatalogMut`].
    ///
    /// Returns the previous [`Handle`] if it existed.
    pub fn insert(&mut self, asset: ResourceKey, handle: Handle<A>) -> Option<Handle<A>> {
        self.0.insert(asset, handle.untyped()).map(UntypedHandle::typed_debug_checked)
    }

    /// Remove an asset from the [`TypedCatalogMut`].
    ///
    /// Returns the [`Handle`] if it existed.
    pub fn remove(&mut self, asset: &str) -> Option<Handle<A>> {
        self.0.remove(asset).map(UntypedHandle::typed_debug_checked)
    }

    /// Remove an asset from the [`TypedCatalogMut`].
    ///
    /// Returns the [`UntypedHandle`] if it existed.
    pub fn remove_untyped(&mut self, asset: &str) -> Option<UntypedHandle> { self.0.remove(asset) }

    /// Returns an [`Entry`] for the asset in the [`TypedCatalogMut`] for
    /// in-place manipulation.
    ///
    /// See [`bevy_utils::hashbrown::hash_map::Entry`] for more information.
    pub fn entry(&mut self, asset: ResourceKey) -> Entry<'_, ResourceKey, UntypedHandle> {
        self.0.entry(asset)
    }

    /// Returns an [`EntryRef`] for the asset in the [`TypedCatalogMut`] for
    /// in-place manipulation.
    ///
    /// See [`bevy_utils::hashbrown::hash_map::EntryRef`] for more information.
    pub fn entry_ref<'b, Q: Equivalent<ResourceKey> + Hash + ?Sized>(
        &mut self,
        asset: &'b Q,
    ) -> EntryRef<'_, 'b, ResourceKey, Q, UntypedHandle, BuildHasherDefault<AHasher>> {
        self.0.entry_ref(asset)
    }

    /// Returns `true` if the [`TypedCatalogMut`] contains the asset.
    #[must_use]
    pub fn contains(&self, asset: &str) -> bool { self.0.contains_key(asset) }

    /// Returns the number of assets in the [`TypedCatalogMut`].
    #[must_use]
    pub fn len(&self) -> usize { self.0.len() }

    /// Returns `true` if the [`TypedCatalogMut`] is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool { self.0.is_empty() }

    /// Returns an iterator over the [`Handle`]s in the [`TypedCatalogMut`].
    ///
    /// # Note
    /// This clones every [`UntypedHandle`] and casts it into a [`Handle`].
    ///
    /// Consider using [`TypedCatalogMut::iter_untyped`] if you only need
    /// [`UntypedHandle`]s.
    #[must_use]
    pub fn iter_typed(&'a self) -> CatalogIter<'a, A, Typed> { CatalogIter::from(self) }

    /// Returns an iterator over the [`UntypedHandle`]s in the
    /// [`TypedCatalogMut`].
    #[must_use]
    pub fn iter_untyped(&'a self) -> CatalogIter<'a, A, Untyped> { CatalogIter::from(self) }

    /// Returns a mutable iterator over the [`UntypedHandle`]s in the
    /// [`TypedCatalogMut`].
    #[must_use]
    pub fn iter_mut_untyped(&'a mut self) -> CatalogIterMut<'a> { CatalogIterMut::from(self) }
}
