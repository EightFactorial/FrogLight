//! TODO
#![allow(dead_code, missing_docs, reason = "WIP")]

use ::core::marker::PhantomData;

pub mod core;
use core::{IndexCore, StrCore};

pub mod location;

pub mod reference;

pub struct IndexedSnbt<'data, C: IndexCore = StrCore<'data>> {
    core: C,
    _phantom: PhantomData<&'data ()>,
}

impl<C: IndexCore> IndexedSnbt<'_, C> {
    /// Create a new [`IndexedSnbt`] from the given [`IndexCore`].
    #[inline]
    #[must_use]
    pub const fn new(core: C) -> Self { Self { core, _phantom: PhantomData } }

    /// Get a reference to the underlying [`IndexCore`].
    #[inline]
    #[must_use]
    pub const fn core(&self) -> &C { &self.core }

    /// Get the raw SNBT data as a string slice.
    #[inline]
    #[must_use]
    pub fn as_str(&self) -> &str { self.core.root() }
}

impl<'data> IndexedSnbt<'data, StrCore<'data>> {
    /// Parse an SNBT structure from the given string slice.
    ///
    /// # Errors
    ///
    /// Returns an error if the string slice is not valid SNBT data.
    #[inline]
    #[expect(clippy::result_unit_err, reason = "WIP")]
    pub fn new_str(str: &'data str) -> Result<Self, ()> { core::parse::parse_snbt(str) }
}
