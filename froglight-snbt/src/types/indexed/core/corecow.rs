use alloc::{borrow::Cow, boxed::Box};

use crate::types::indexed::core::{IndexCore, StrCore};

#[derive(Debug)]
pub struct CowCore<'data> {
    data: Cow<'data, str>,
    entries: Box<[()]>,
}

impl<'data> CowCore<'data> {
    /// Creates a new [`CowCore`] from the given [`StrCore`].
    #[inline]
    #[must_use]
    #[expect(clippy::should_implement_trait, reason = "Different meaning")]
    pub fn from_str(core: StrCore<'data>) -> Self {
        Self { data: Cow::Borrowed(core.data), entries: core.entries }
    }
}

impl IndexCore for CowCore<'_> {
    fn root(&self) -> &str { self.data.as_ref() }

    fn entries(&self) -> &[()] { &self.entries }
}
