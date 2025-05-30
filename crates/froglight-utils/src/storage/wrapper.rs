//! TODO

#[cfg(feature = "reflect")]
use bevy_reflect::prelude::*;
use derive_more::Deref;
use downcast_rs::Downcast;

/// A wrapper around a static value that implements
/// [`PartialEq`] and [`Eq`] based on its [`TypeId`].
#[derive(Debug, Deref)]
#[cfg_attr(feature = "reflect", derive(Reflect), reflect(Clone, PartialEq))]
pub struct StorageWrapper<V: Downcast + ?Sized + 'static>(&'static V);

impl<V: Downcast + ?Sized + 'static> StorageWrapper<V> {
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

impl<V: Downcast + ?Sized + 'static> Clone for StorageWrapper<V> {
    fn clone(&self) -> Self { *self }
}
impl<V: Downcast + ?Sized + 'static> Copy for StorageWrapper<V> {}

impl<V: Downcast + ?Sized + 'static> Eq for StorageWrapper<V> {}
impl<V: Downcast + ?Sized + 'static> PartialEq for StorageWrapper<V> {
    fn eq(&self, other: &Self) -> bool {
        <V as Downcast>::as_any(self.0).type_id() == <V as Downcast>::as_any(other.0).type_id()
    }
}
