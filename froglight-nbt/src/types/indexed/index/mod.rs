//! TODO

use core::{any::TypeId, fmt, hash, marker::PhantomData};

mod entry;
pub use entry::EntryIndex;

mod value;
pub use value::ValueIndex;

/// A typed index for an NBT entry.
#[repr(transparent)]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct Index<T: ?Sized>(usize, PhantomData<T>);

impl<T: ?Sized> Index<T> {
    /// Create a new typed [`Index`].
    #[inline]
    #[must_use]
    pub const fn new(index: usize) -> Self { Self(index, PhantomData) }

    /// Cast this [`Index`] to a different type.
    ///
    /// # Safety
    ///
    /// The caller must ensure that index value points to a valid entry of type
    /// `U`.
    #[inline]
    #[must_use]
    pub const unsafe fn cast<U: ?Sized>(self) -> Index<U> { Index(self.0, PhantomData) }

    /// Get the raw index value.
    #[inline]
    #[must_use]
    pub const fn value(self) -> usize { self.0 }
}

// -------------------------------------------------------------------------------------------------

impl<T: ?Sized> fmt::Debug for Index<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Index").field(&self.0).finish()
    }
}
impl<T: ?Sized> fmt::Display for Index<T> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { fmt::Display::fmt(&self.0, f) }
}

impl<T: ?Sized> Clone for Index<T> {
    #[inline]
    fn clone(&self) -> Self { *self }
}
impl<T: ?Sized> Copy for Index<T> {}

impl<T: ?Sized> PartialEq for Index<T> {
    #[inline]
    fn eq(&self, other: &Self) -> bool { self.0 == other.0 }
}
impl<T: ?Sized> Eq for Index<T> {}

impl<T: ?Sized> PartialOrd for Index<T> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> { Some(self.cmp(other)) }
}
impl<T: ?Sized> Ord for Index<T> {
    #[inline]
    fn cmp(&self, other: &Self) -> core::cmp::Ordering { self.0.cmp(&other.0) }
}

impl<T: ?Sized + 'static> hash::Hash for Index<T> {
    #[inline]
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        TypeId::of::<T>().hash(state);
        self.0.hash(state);
    }
}

impl<T: ?Sized> From<usize> for Index<T> {
    #[inline]
    fn from(value: usize) -> Self { Self::new(value) }
}
impl<T: ?Sized> From<Index<T>> for usize {
    #[inline]
    fn from(index: Index<T>) -> Self { index.0 }
}
