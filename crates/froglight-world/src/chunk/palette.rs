#[cfg(feature = "io")]
use froglight_io::prelude::*;

/// A palette for a [`Section`](super::Section).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SectionPalette {
    /// All items in the section have the same id.
    Single(u32),
    /// All items in the section index into this vector.
    Vector(Vec<u32>),
    /// Use the global palette.
    Global,
}

impl SectionPalette {
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
    fn default() -> Self { SectionPalette::Single(0) }
}

// -------------------------------------------------------------------------------------------------

#[cfg(feature = "io")]
impl SectionPalette {
    /// Read a [`SectionPalette`] from a buffer, given an expected palette type.
    ///
    /// # Errors
    /// Returns an error if the type does not match the buffer,
    /// or if the buffer fails to be read from.
    pub fn frog_read(&self, buffer: &mut impl std::io::Read) -> Result<Self, ReadError> {
        match self {
            SectionPalette::Single(..) => {
                Ok(SectionPalette::Single(FrogVarRead::frog_var_read(buffer)?))
            }
            SectionPalette::Vector(..) => {
                Ok(SectionPalette::Vector(FrogVarRead::frog_var_read(buffer)?))
            }
            SectionPalette::Global => Ok(SectionPalette::Global),
        }
    }
}

#[cfg(feature = "io")]
impl FrogWrite for SectionPalette {
    fn frog_write(&self, buffer: &mut impl std::io::Write) -> Result<usize, WriteError> {
        match self {
            SectionPalette::Single(value) => FrogVarWrite::frog_var_write(value, buffer),
            SectionPalette::Vector(values) => FrogVarWrite::frog_var_write(values, buffer),
            SectionPalette::Global => Ok(0),
        }
    }

    fn frog_len(&self) -> usize {
        match self {
            SectionPalette::Single(value) => FrogVarWrite::frog_var_len(value),
            SectionPalette::Vector(values) => FrogVarWrite::frog_var_len(values),
            SectionPalette::Global => 0,
        }
    }
}
