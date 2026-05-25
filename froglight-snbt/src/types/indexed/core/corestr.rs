use alloc::boxed::Box;

use crate::types::indexed::core::IndexCore;

#[derive(Debug)]
pub struct StrCore<'data> {
    pub(super) data: &'data str,
    pub(super) entries: Box<[()]>,
}

impl<'data> StrCore<'data> {
    /// Creates a new [`StrCore`] from the given data and entries.
    ///
    /// # SAFETY
    ///
    /// TODO
    #[inline]
    #[must_use]
    pub const unsafe fn new(data: &'data str, entries: Box<[()]>) -> Self { Self { data, entries } }
}

impl IndexCore for StrCore<'_> {
    fn root(&self) -> &str { self.data }

    fn entries(&self) -> &[()] { &self.entries }
}
