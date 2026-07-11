//! TODO

mod corecow;
pub use corecow::CowCore;

mod coreslice;
pub use coreslice::SliceCore;

mod debug;

pub(super) mod parse;

use super::{IndexedNbt, Mut, Ref};

/// A type alias for an [`IndexedNbt`] with a [`SliceCore`] core.
pub type IndexedNbtSlice<'a, A = Ref> = IndexedNbt<'a, A, SliceCore<'a, A>>;

/// A type alias for an [`IndexedNbt`] with a [`CowCore`] core.
pub type IndexedNbtCow<'a, A = Mut> = IndexedNbt<'a, A, CowCore<'a, A>>;
