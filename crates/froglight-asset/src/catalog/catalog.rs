use std::any::TypeId;

use bevy_asset::{Asset, Handle, UntypedHandle};
use bevy_derive::{Deref, DerefMut};
use bevy_ecs::{reflect::ReflectResource, system::Resource};
use bevy_reflect::Reflect;
use bevy_utils::{HashMap, TypeIdMap};
use froglight_common::ResourceKey;

use super::{TypedCatalogMut, TypedCatalogRef};

/// A collection of [`Asset`]s that can be accessed by name.
#[derive(Debug, Default, Clone, PartialEq, Eq, Resource, Reflect)]
#[reflect(Resource)]
pub struct AssetCatalog(TypeIdMap<UntypedAssetMap>);

impl AssetCatalog {
    /// Get a [`Handle`] to an [`Asset`] from the [`AssetCatalog`],
    /// if it exists.
    #[must_use]
    pub fn get<A: Asset>(&self, asset: &str) -> Option<Handle<A>> {
        let untyped = self.0.get(&TypeId::of::<A>())?;
        untyped.get(asset).map(|h| h.clone().typed_debug_checked())
    }

    /// Get an [`UntypedHandle`] to an [`Asset`] from the [`AssetCatalog`],
    /// if it exists.
    #[must_use]
    pub fn get_untyped<A: Asset>(&self, asset: &str) -> Option<&UntypedHandle> {
        self.0.get(&TypeId::of::<A>()).and_then(|m| m.get(asset))
    }

    /// Insert an [`Asset`] into the [`AssetCatalog`].
    pub fn insert<A: Asset>(&mut self, asset: ResourceKey, handle: Handle<A>) {
        let untyped = self.0.entry(TypeId::of::<A>()).or_default();
        untyped.insert(asset, handle.untyped());
    }

    /// Returns `true` if the [`AssetCatalog`] contains the asset.
    #[must_use]
    pub fn contains<A: Asset>(&self, asset: &str) -> bool {
        self.0.get(&TypeId::of::<A>()).map_or(false, |m| m.contains_key(asset))
    }

    /// Returns the number of asset types in the [`AssetCatalog`].
    #[must_use]
    pub fn len(&self) -> usize { self.0.len() }

    /// Returns the total number of assets in the [`AssetCatalog`].
    #[must_use]
    pub fn len_total(&self) -> usize { self.0.values().map(|m| m.len()).sum() }

    /// Returns the number of assets of type `A` in the [`AssetCatalog`].
    #[must_use]
    pub fn len_of<A: Asset>(&self) -> usize {
        self.0.get(&TypeId::of::<A>()).map_or(0, |m| m.len())
    }

    /// Returns `true` if the [`AssetCatalog`] is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool { self.0.is_empty() }

    /// Returns `true` if the [`AssetCatalog`] contains no assets of type `A`.
    #[must_use]
    pub fn is_empty_of<A: Asset>(&self) -> bool {
        self.0.get(&TypeId::of::<A>()).map_or(true, |m| m.is_empty())
    }
    /// Clear the [`AssetCatalog`].
    ///
    /// This will remove all assets from the [`AssetCatalog`].
    pub fn clear(&mut self) { self.0.clear() }

    /// Clear the [`AssetCatalog`] of a specific [`Asset`] type.
    ///
    /// This will remove all assets of the specified type from the
    /// [`AssetCatalog`].
    pub fn clear_of<A: Asset>(&mut self) {
        if let Some(m) = self.0.get_mut(&TypeId::of::<A>()) {
            m.clear();
        }
    }

    /// Iterate over all [`UntypedHandle`]s in the [`AssetCatalog`].
    pub fn iter(&self) -> impl Iterator<Item = (&ResourceKey, &UntypedHandle, &TypeId)> {
        self.0
            .iter()
            .flat_map(|(type_id, m)| m.iter().map(move |(key, handle)| (key, handle, type_id)))
    }
    /// Get a reference to the [`AssetCatalog`] for an [`Asset`].
    ///
    /// This is useful when reading many assets of the same type.
    #[must_use]
    pub fn typed_ref<A: Asset>(&self) -> Option<TypedCatalogRef<A>> {
        self.0.get(&TypeId::of::<A>()).map(TypedCatalogRef::new)
    }

    /// Get a mutable reference to the [`AssetCatalog`] for an [`Asset`].
    ///
    /// This is useful when modifying many assets of the same type.
    ///
    /// # Note
    /// This will create an empty entry for the [`Asset`] type if it does not
    /// exist.
    #[must_use]
    pub fn typed_mut<A: Asset>(&mut self) -> TypedCatalogMut<A> {
        TypedCatalogMut::new(self.0.entry(TypeId::of::<A>()).or_default())
    }

    /// Get a reference to the [`AssetCatalog`] for an [`Asset`] type.
    ///
    /// This is useful when needing to read and write assets of different types
    /// at the same time.
    ///
    /// # Note
    /// This will create an empty entry for the [`Asset`] type if it does not
    /// exist.
    pub fn typed_mut_scope<A: Asset>(
        &mut self,
        fun: impl FnOnce(&mut AssetCatalog, TypedCatalogMut<A>),
    ) {
        let mut taken = std::mem::take(self.0.entry(TypeId::of::<A>()).or_default());
        fun(self, TypedCatalogMut::new(&mut taken));
        self.0.insert(TypeId::of::<A>(), taken);
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Deref, DerefMut, Reflect)]
pub(super) struct UntypedAssetMap(#[reflect(ignore)] HashMap<ResourceKey, UntypedHandle>);
