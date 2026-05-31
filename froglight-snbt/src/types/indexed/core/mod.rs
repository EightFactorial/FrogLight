//! TODO

use core::range::Range;

mod corecow;
pub use corecow::CowCore;

mod coreslice;
pub use coreslice::SliceCore;

/// A trait for an index of SNBT entries.
pub trait IndexCore {
    /// Get the root string.
    #[must_use]
    fn root(&self) -> &str;

    /// Get a slice of [`Entries`](Entry).
    ///
    /// # Safety
    ///
    /// The caller must ensure that the range is valid.
    #[must_use]
    unsafe fn get_entries(&self, range: Range<usize>) -> &[()];
}
