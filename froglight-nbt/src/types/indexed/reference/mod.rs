//! TODO

use crate::types::indexed::{
    core::{Mut, NbtAccess},
    index::Index,
};

mod slice;

mod value;
pub use value::IndexedValueReference;

/// A type that accessed via an [`Index`].
pub struct IndexedReference<'data, T: IndexableValue + ?Sized, A: NbtAccess> {
    slice: A::SLICE<'data>,
    index: Index<T>,
}

impl<'data, T: IndexableValue + ?Sized, A: NbtAccess> IndexedReference<'data, T, A> {
    /// Create a new [`IndexedReference`] from the given slice and index.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the index is valid for the given slice.
    #[inline]
    #[must_use]
    pub const unsafe fn new(slice: A::SLICE<'data>, index: Index<T>) -> Self {
        Self { slice, index }
    }

    /// Get the value held by this reference.
    #[must_use]
    pub fn get(&self) -> T::Value<'_> {
        // SAFETY: `IndexedReference` guarantees that this is safe
        unsafe { T::get(self.slice.as_ref(), self.index) }
    }
}

impl<T: IndexableValueMut + ?Sized> IndexedReference<'_, T, Mut> {
    /// Set the value held by this reference.
    pub fn set(&mut self, value: T::Value<'_>) {
        // SAFETY: `IndexedReference` guarantees that this is safe
        unsafe { T::set(self.slice, self.index, value) }
    }
}

// -------------------------------------------------------------------------------------------------

/// A type that can be accessed via an [`Index`].
///
/// # Safety
///
/// The implementer must ensure that [`INDEX_IS_ENTRY_RANGE`] is correct,
/// otherwise it will cause incorrect and undefined behavior.
pub unsafe trait IndexableValue: sealed::Sealed {
    /// The type of value that is accessed via an [`Index`].
    type Value<'data>: Sized;

    /// Whether a list's index is an index into the data slice,
    /// or an index into the list of entry ranges.
    const LIST_INDEX_IS_ENTRY_RANGE: bool;

    /// Get the size of the whole value using the given slice and index.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the index is valid for the given slice.
    #[must_use]
    unsafe fn size(slice: &[u8], index: Index<Self>) -> usize;

    /// Get the value using the given slice and index.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the index is valid for the given slice.
    #[must_use]
    unsafe fn get(slice: &[u8], index: Index<Self>) -> Self::Value<'_>;
}

/// A type that can be mutated via an [`Index`].
///
/// Requires that the type also implements [`IndexableValue`].
pub trait IndexableValueMut: IndexableValue {
    /// Set the value using the given slice and index.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the index is valid for the given slice.
    unsafe fn set(slice: &mut [u8], index: Index<Self>, value: Self::Value<'_>);
}

/// A slice-type that can be accessed via an [`Index`].
///
/// Requires that the type also implements [`IndexableValue`].
pub trait IndexableSlice: IndexableValue {
    /// The size of the size prefix, in bytes.
    const SIZE_BYTES: usize;

    /// Get the length of the slice using the given slice and index.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the index is valid for the given slice.
    #[must_use]
    unsafe fn slice_len(slice: &[u8], index: Index<Self>) -> usize;
}

// -------------------------------------------------------------------------------------------------

mod sealed {
    use froglight_mutf8::prelude::MStr;

    pub trait Sealed {}

    impl Sealed for u8 {}
    impl Sealed for u16 {}
    impl Sealed for u32 {}
    impl Sealed for u64 {}
    impl Sealed for f32 {}
    impl Sealed for f64 {}

    impl Sealed for [u8] {}
    impl Sealed for [u32] {}
    impl Sealed for [u64] {}
    impl Sealed for MStr {}
}
