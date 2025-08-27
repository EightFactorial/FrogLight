//! TODO

use alloc::vec::Vec;

/// A palette for a [`Section`](crate::section::Section).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SectionPalette {
    /// All entries in the section have the same id.
    Single(u32),
    /// All entries in the section index into this vector.
    Vector(Vec<u32>),
    /// All entries in the section are stored as-is.
    Global,
}

impl SectionPalette {
    /// A palette representing a [`Section`](crate::section::Section) full of
    /// air.
    pub const AIR: Self = SectionPalette::Single(0);

    /// A palette representing a [`Section`](crate::section::Section) full of
    /// air.
    #[must_use]
    pub const fn air() -> Self { Self::AIR }

    /// Returns `true` if the [`SectionPalette`] is [`SectionPalette::Single`].
    #[must_use]
    pub const fn is_single(&self) -> bool { matches!(self, SectionPalette::Single(..)) }

    /// Returns `true` if the [`SectionPalette`] is [`SectionPalette::Vector`].
    #[must_use]
    pub const fn is_vector(&self) -> bool { matches!(self, SectionPalette::Vector(..)) }

    /// Returns `true` if the [`SectionPalette`] is [`SectionPalette::Global`].
    #[must_use]
    pub const fn is_global(&self) -> bool { matches!(self, SectionPalette::Global) }
}

impl Default for SectionPalette {
    fn default() -> Self { Self::AIR }
}
