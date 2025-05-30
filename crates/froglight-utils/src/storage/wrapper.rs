//! TODO

use core::fmt::Debug;

#[cfg(feature = "reflect")]
use bevy_reflect::prelude::*;
use derive_more::Deref;
use downcast_rs::Downcast;

/// A wrapper around a static value that implements
/// [`PartialEq`] and [`Eq`] based on its [`TypeId`].
#[derive(Deref)]
#[cfg_attr(feature = "reflect", derive(Reflect), reflect(Clone, PartialEq))]
pub struct StorageWrapper<V: ?Sized + 'static>(&'static V);

impl<V: ?Sized + 'static> StorageWrapper<V> {
    /// Create a new [`StorageWrapper`] from the given value.
    #[inline]
    #[must_use]
    pub const fn new(value: &'static V) -> Self { Self(value) }

    /// Get a reference to the inner static value.
    #[inline]
    #[must_use]
    pub const fn inner(&self) -> &'static V { self.0 }
}

// -------------------------------------------------------------------------------------------------
// Manual implementations for `StorageWrapper` to avoid trait bounds

impl<V: Debug + ?Sized + 'static> Debug for StorageWrapper<V> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result { self.inner().fmt(f) }
}

impl<V: ?Sized + 'static> Clone for StorageWrapper<V> {
    fn clone(&self) -> Self { *self }
}
impl<V: ?Sized + 'static> Copy for StorageWrapper<V> {}

impl<V: ?Sized + 'static> Eq for StorageWrapper<V> {}
impl<V: ?Sized + 'static> PartialEq for StorageWrapper<V> {
    fn eq(&self, other: &Self) -> bool { self.0.as_any().type_id() == other.0.as_any().type_id() }
}
