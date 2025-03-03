//! TODO

use crate::nbt::{NbtCompound, UnnamedNbt};

/// A trait for converting between a type and NBT.
pub trait ConvertNbt: Sized {
    /// Parse the type from an [`NbtCompound`].
    ///
    /// # Errors
    /// Returns an error if the type could not be parsed
    /// from the given [`NbtCompound`].
    fn from_compound(nbt: &NbtCompound) -> Result<Self, ConvertError>;

    /// Convert the type into an [`NbtCompound`].
    ///
    /// # Errors
    /// Returns an error if the type could not be converted
    /// into an [`NbtCompound`].
    fn as_compound(&self) -> Result<NbtCompound, ConvertError>;

    /// Convert the type into an [`UnnamedNbt`].
    ///
    /// # Errors
    /// Returns an error if the type could not be converted
    /// into an [`UnnamedNbt`].
    #[inline]
    fn as_nbt(&self) -> Result<UnnamedNbt, ConvertError> { self.as_compound().map(UnnamedNbt::new) }
}

/// An error that can occur when converting between a type and NBT.
#[derive(Debug, thiserror::Error)]
pub enum ConvertError {
    /// An error occurred while parsing content formats.
    #[error("Invalid format: expected '{0}', found '{1}'")]
    InvalidFormat(char, char),
    /// Unexpected data was left over after parsing.
    #[error("Unexpected data remaining: \"{0}\"")]
    UnexpectedData(String),

    /// An error occurred while parsing a type from a string.
    #[error("Failed to parse type: \"{0}\"")]
    FromString(&'static str),
}
