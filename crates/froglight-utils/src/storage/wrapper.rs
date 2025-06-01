//! TODO

use core::{any::Any, fmt::Debug};

#[cfg(feature = "reflect")]
use bevy_reflect::prelude::*;
use derive_more::Deref;
use downcast_rs::Downcast;

/// A wrapper around a static value that implements
/// [`PartialEq`] and [`Eq`] based on its [`TypeId`](core::any::TypeId).
#[derive(Deref)]
#[cfg_attr(feature = "reflect", derive(Reflect), reflect(Clone))]
pub struct StorageWrapper<T: ?Sized + 'static>(&'static T);

impl<T: ?Sized + 'static> StorageWrapper<T> {
    /// Create a new [`StorageWrapper`] from the given value.
    #[inline]
    #[must_use]
    pub const fn new(value: &'static T) -> Self { Self(value) }

    /// Get a reference to the inner static value.
    #[inline]
    #[must_use]
    pub const fn inner(&self) -> &'static T { self.0 }

    /// Get the inner value as a reference to `dyn Any`.
    #[inline]
    #[must_use]
    pub fn as_any(&self) -> &dyn Any
    where T: Downcast {
        <T as Downcast>::as_any(self.0)
    }
}

// -------------------------------------------------------------------------------------------------
// Manual implementations for `StorageWrapper` to avoid trait bounds

impl<T: Debug + ?Sized + 'static> Debug for StorageWrapper<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result { self.inner().fmt(f) }
}

impl<T: ?Sized + 'static> Clone for StorageWrapper<T> {
    fn clone(&self) -> Self { *self }
}
impl<T: ?Sized + 'static> Copy for StorageWrapper<T> {}

impl<T: Downcast + ?Sized + 'static> Eq for StorageWrapper<T> {}
impl<T: Downcast + ?Sized + 'static> PartialEq for StorageWrapper<T> {
    fn eq(&self, other: &Self) -> bool {
        <T as Downcast>::as_any(self.0).type_id() == <T as Downcast>::as_any(other.0).type_id()
    }
}
