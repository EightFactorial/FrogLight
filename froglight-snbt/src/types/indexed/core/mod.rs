//! TODO

// use crate::types::indexed::index::EntryIndex;

mod corecow;
pub use corecow::CowCore;

mod corestr;
pub use corestr::StrCore;

mod debug;
pub(super) mod parse;

/// A trait for an index of SNBT entries.
pub trait IndexCore {
    /// Get a reference to the root SNBT data slice.
    #[must_use]
    fn root(&self) -> &str;

    /// Get a reference to the [`EntryIndex`]es of this SNBT structure.
    #[must_use]
    fn entries(&self) -> &[()];
}
