use bevy_asset::{Asset, Handle, UntypedHandle};
use bevy_utils::hashbrown::hash_map::IterMut;
use froglight_common::ResourceKey;

use super::{TypedCatalogMut, TypedCatalogRef};

/// An iterator over the [`Asset`]s in an [`AssetCatalog`](super::AssetCatalog).
// TODO: Write iterator that doesn't use `Box`
pub struct CatalogIter<'a, A: Asset, K: IteratorType<'a, A>>(
    Box<dyn Iterator<Item = (&'a ResourceKey, <K as IteratorType<'a, A>>::Kind)> + 'a>,
);
impl<'a, A: Asset, K: IteratorType<'a, A>> Iterator for CatalogIter<'a, A, K> {
    type Item = (&'a ResourceKey, <K as IteratorType<'a, A>>::Kind);
    fn next(&mut self) -> Option<Self::Item> { self.0.next() }
}

// Create `Untyped` implementations for `CatalogIter`.

impl<'a, A: Asset> From<TypedCatalogRef<'a, A>> for CatalogIter<'a, A, Untyped> {
    fn from(value: TypedCatalogRef<'a, A>) -> Self { Self(Box::new(value.0.iter())) }
}
impl<'a, A: Asset> From<&'a TypedCatalogRef<'a, A>> for CatalogIter<'a, A, Untyped> {
    fn from(value: &'a TypedCatalogRef<'a, A>) -> Self { Self(Box::new(value.0.iter())) }
}
impl<'a, A: Asset> From<TypedCatalogMut<'a, A>> for CatalogIter<'a, A, Untyped> {
    fn from(value: TypedCatalogMut<'a, A>) -> Self { Self(Box::new(value.0.iter())) }
}
impl<'a, A: Asset> From<&'a TypedCatalogMut<'a, A>> for CatalogIter<'a, A, Untyped> {
    fn from(value: &'a TypedCatalogMut<'a, A>) -> Self { Self(Box::new(value.0.iter())) }
}

// Create `Typed` implementations for `CatalogIter`.

impl<'a, A: Asset> CatalogIter<'a, A, Typed> {
    fn map_fn((k, v): (&'a ResourceKey, &'a UntypedHandle)) -> (&'a ResourceKey, Handle<A>) {
        (k, v.clone().typed_debug_checked::<A>())
    }
}

impl<'a, A: Asset> From<TypedCatalogRef<'a, A>> for CatalogIter<'a, A, Typed> {
    fn from(value: TypedCatalogRef<'a, A>) -> Self {
        Self(Box::new(value.0.iter().map(Self::map_fn)))
    }
}
impl<'a, A: Asset> From<&'a TypedCatalogRef<'a, A>> for CatalogIter<'a, A, Typed> {
    fn from(value: &'a TypedCatalogRef<'a, A>) -> Self {
        Self(Box::new(value.0.iter().map(Self::map_fn)))
    }
}
impl<'a, A: Asset> From<TypedCatalogMut<'a, A>> for CatalogIter<'a, A, Typed> {
    fn from(value: TypedCatalogMut<'a, A>) -> Self {
        Self(Box::new(value.0.iter().map(Self::map_fn)))
    }
}
impl<'a, A: Asset> From<&'a TypedCatalogMut<'a, A>> for CatalogIter<'a, A, Typed> {
    fn from(value: &'a TypedCatalogMut<'a, A>) -> Self {
        Self(Box::new(value.0.iter().map(Self::map_fn)))
    }
}

/// A mutable iterator over the [`Asset`]s in an
/// [`AssetCatalog`](super::AssetCatalog).
pub struct CatalogIterMut<'a>(IterMut<'a, ResourceKey, UntypedHandle>);
impl<'a> Iterator for CatalogIterMut<'a> {
    type Item = (&'a ResourceKey, &'a mut UntypedHandle);
    fn next(&mut self) -> Option<Self::Item> { self.0.next() }
}

// Create `Untyped` implementations for `CatalogIterMut`.

impl<'a, A: Asset> From<TypedCatalogMut<'a, A>> for CatalogIterMut<'a> {
    fn from(value: TypedCatalogMut<'a, A>) -> Self { Self(value.0.iter_mut()) }
}
impl<'a, A: Asset> From<&'a mut TypedCatalogMut<'a, A>> for CatalogIterMut<'a> {
    fn from(value: &'a mut TypedCatalogMut<'a, A>) -> Self { Self(value.0.iter_mut()) }
}

pub(super) use sealed::{IteratorType, Typed, Untyped};
mod sealed {
    #![allow(unreachable_pub)]

    use bevy_asset::{Asset, Handle, UntypedHandle};

    pub trait IteratorType<'a, A> {
        type Kind;
    }

    pub struct Typed;
    impl<A: Asset> IteratorType<'_, A> for Typed {
        type Kind = Handle<A>;
    }

    pub struct Untyped;
    impl<'a, A: Asset> IteratorType<'a, A> for Untyped {
        type Kind = &'a UntypedHandle;
    }
}
