//! TODO

mod corecow;
pub use corecow::CowCore;

mod coreslice;
pub use coreslice::SliceCore;

mod debug;

pub(super) mod parse;

use super::{IndexedNbt, Ref};

/// A type alias for an [`IndexedNbt`] with a [`SliceCore`] core.
pub type IndexedNbtSlice<'a> = IndexedNbt<SliceCore<'a, Ref>>;

/// A type alias for an [`IndexedNbt`] with a [`CowCore`] core.
pub type IndexedNbtCow<'a> = IndexedNbt<CowCore<'a>>;
