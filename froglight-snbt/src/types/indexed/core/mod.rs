//! TODO

use core::range::Range;

mod corecow;
pub use corecow::CowCore;

mod coreslice;
pub use coreslice::SliceCore;

use crate::types::indexed::{IndexedSnbt, entry::EntryIndex};

/// A type alias for an [`IndexedSnbt`] with a [`SliceCore`] core.
pub type IndexedSnbtSlice<'core> = IndexedSnbt<SliceCore<'core>>;

/// A type alias for an [`IndexedSnbt`] with a [`CowCore`] core.
pub type IndexedSnbtCow<'core> = IndexedSnbt<CowCore<'core>>;

/// A trait for an index of SNBT entries.
pub trait IndexCore {
    /// The long-lived root string type.
    type RootLong<'a>
    where
        Self: 'a;

    /// Get the root string.
    #[must_use]
    fn root(&self) -> &str;

    /// Get the long-lived root string.
    #[must_use]
    fn root_long(&self) -> Self::RootLong<'_>;

    /// Get a slice of [`Entries`](Entry).
    ///
    /// # Safety
    ///
    /// The caller must ensure that the range is valid.
    #[must_use]
    unsafe fn get_entries(&self, range: Range<usize>) -> &[EntryIndex];
}
