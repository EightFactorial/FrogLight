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

impl ConvertNbt for NbtCompound {
    fn from_compound(nbt: &NbtCompound) -> Result<Self, ConvertError> { Ok(nbt.clone()) }

    fn as_compound(&self) -> Result<NbtCompound, ConvertError> { Ok(self.clone()) }
}

/// An error that can occur when converting between a type and NBT.
#[derive(Debug, thiserror::Error)]
pub enum ConvertError {
    /// A field was missing from the NBT compound.
    #[error("Missing field: \"{0}\"")]
    MissingField(&'static str),
    /// A field's tag did not match the expected tag.
    #[error("Mismatched tag for: \"{0}\"")]
    MismatchedTag(&'static str),
    /// An error occurred while converting a field.
    #[error("Failed to create \"{0}\": {1}")]
    ConversionError(&'static str, Box<dyn std::error::Error>),
}
