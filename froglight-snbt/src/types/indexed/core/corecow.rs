use alloc::{borrow::Cow, boxed::Box};
use core::range::Range;

use crate::types::indexed::{
    core::{IndexCore, StrCore},
    index::EntryIndex,
};

pub struct CowCore<'data> {
    data: Cow<'data, str>,
    entries: Box<[EntryIndex]>,
    ranges: Box<[Range<usize>]>,
}

impl<'data> CowCore<'data> {
    /// Creates a new [`CowCore`] from the given [`StrCore`].
    #[inline]
    #[must_use]
    #[expect(clippy::should_implement_trait, reason = "Different meaning")]
    pub fn from_str(core: StrCore<'data>) -> Self {
        Self { data: Cow::Borrowed(core.data), entries: core.entries, ranges: core.ranges }
    }
}

impl IndexCore for CowCore<'_> {
    fn root(&self) -> &str { self.data.as_ref() }

    fn entries(&self) -> &[EntryIndex] { &self.entries }

    unsafe fn entry_range(&self, index: usize) -> &[EntryIndex] {
        // SAFETY: The caller ensures that this is safe.
        unsafe {
            let range = self.ranges.get_unchecked(index);
            self.entries.get_unchecked(*range)
        }
    }
}
