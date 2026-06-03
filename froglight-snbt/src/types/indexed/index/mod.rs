//! TODO

use core::{fmt, range::Range};

use crate::types::indexed::{core::IndexCore, entry::EntryIndex};

pub mod bool;
pub mod numeric;
pub mod slice;
pub mod snbt;
pub mod string;

/// An index and description of an SNBT value.
pub struct Index<T: Indexable + ?Sized> {
    range: Range<usize>,
    description: T::Description,
}

/// A trait for types that can be [`Index`]ed.
pub trait Indexable {
    /// A description of the indexed value.
    type Description: fmt::Debug + Copy + Sized;
}

/// A trait for types that can be read from an [`Index`].
pub trait IndexableValue: Indexable {
    /// The type of the return value.
    type Value<'a>: Sized;

    /// Read an [`Index`]ed value from the given root string.
    ///
    /// # Safety
    ///
    /// The caller must ensure the [`Index`] is valid for the given root string.
    #[must_use]
    unsafe fn read_value(index: Index<Self>, root: &str) -> Self::Value<'_>;
}

/// A trait for types that refer to a slice of [`Entries`](Entry).
pub trait IndexableSlice: Indexable {
    /// Get the slice of [`Entries`](Entry) that the [`Index`] points to.
    ///
    /// # Safety
    ///
    /// The caller must ensure the [`Index`] is valid for the given core.
    #[must_use]
    unsafe fn read_entries<C: IndexCore>(index: Index<Self>, core: &C) -> &[EntryIndex];
}

impl<T: Indexable + ?Sized> Index<T> {
    /// Create a new [`Index`] with the given range and description.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the value is valid,
    /// and matches the description.
    #[inline]
    #[must_use]
    pub const unsafe fn new(range: Range<usize>, description: T::Description) -> Self {
        Self { range, description }
    }

    /// Create a new [`Index`] from a string slice, starting index, and
    /// description.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the value is valid,
    /// and matches the description.
    #[inline]
    #[must_use]
    pub const unsafe fn new_from(slice: &str, start: usize, settings: T::Description) -> Self {
        // SAFETY: The caller ensures that this is safe.
        unsafe { Self::new(Range { start, end: start + slice.len() }, settings) }
    }

    /// Get the range of the indexed value.
    #[inline]
    #[must_use]
    pub const fn range(self) -> Range<usize> { self.range }

    /// Get a description of the indexed value.
    #[inline]
    #[must_use]
    pub const fn description(self) -> T::Description { self.description }
}

impl<T: IndexableValue + ?Sized> Index<T> {
    /// Read the value from the given root string.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the [`Index`] is valid for the given root
    /// string.
    #[inline]
    #[must_use]
    pub unsafe fn read_value(self, root: &str) -> T::Value<'_> {
        // SAFETY: The caller ensures that this is safe.
        unsafe { T::read_value(self, root) }
    }
}

impl<T: IndexableSlice + ?Sized> Index<T> {
    /// Get the slice of [`Entries`](Entry) that the [`Index`] points to.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the [`Index`] is valid for the given core.
    #[inline]
    #[must_use]
    pub unsafe fn read_entries<C: IndexCore>(self, core: &C) -> &[EntryIndex] {
        // SAFETY: The caller ensures that this is safe.
        unsafe { T::read_entries(self, core) }
    }
}

// -------------------------------------------------------------------------------------------------

impl<T: Indexable + ?Sized> fmt::Debug for Index<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Index")
            .field("range", &self.range)
            .field("description", &self.description)
            .finish()
    }
}

impl<T: Indexable + ?Sized> Clone for Index<T> {
    #[inline]
    fn clone(&self) -> Self { *self }
}
impl<T: Indexable + ?Sized> Copy for Index<T> {}

impl<T: Indexable + ?Sized> PartialEq for Index<T> {
    #[inline]
    fn eq(&self, other: &Self) -> bool { self.range == other.range }
}
impl<T: Indexable + ?Sized> Eq for Index<T> {}

impl<T: Indexable + ?Sized> PartialOrd for Index<T> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> { Some(self.cmp(other)) }
}
impl<T: Indexable + ?Sized> Ord for Index<T> {
    #[inline]
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.range.start.cmp(&other.range.start).then(self.range.end.cmp(&other.range.end))
    }
}
