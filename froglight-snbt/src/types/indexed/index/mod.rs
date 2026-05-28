//! TODO

use core::fmt;

pub mod numeric;
pub mod string;

/// An index and description of an SNBT value.
pub struct Index<T: Indexable + ?Sized> {
    start: usize,
    length: usize,
    description: T::Description,
}

/// A trait for types that can be [`Index`]ed.
pub trait Indexable {
    /// A description of the indexed value.
    type Description: fmt::Debug + Copy + Sized;
}

impl<T: Indexable + ?Sized> Index<T> {
    /// Create a new [`Index`] with the given starting index, length, and
    /// description.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the value is valid,
    /// and matches the description.
    #[inline]
    #[must_use]
    pub const unsafe fn new(start: usize, length: usize, description: T::Description) -> Self {
        Self { start, length, description }
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
        unsafe { Self::new(start, slice.len(), settings) }
    }
}

// -------------------------------------------------------------------------------------------------

impl<T: Indexable + ?Sized> fmt::Debug for Index<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Index")
            .field("start", &self.start)
            .field("length", &self.length)
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
    fn eq(&self, other: &Self) -> bool { self.start == other.start && self.length == other.length }
}
impl<T: Indexable + ?Sized> Eq for Index<T> {}

impl<T: Indexable + ?Sized> PartialOrd for Index<T> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> { Some(self.cmp(other)) }
}
impl<T: Indexable + ?Sized> Ord for Index<T> {
    #[inline]
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.start.cmp(&other.start).then(self.length.cmp(&other.length))
    }
}
