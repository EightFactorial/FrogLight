//! TODO

use crate::nbt::{NbtCompound, UnnamedNbt};

/// A trait for converting between a type and NBT.
pub trait ConvertNbt: Sized {
    /// Parse the type from [`NbtCompound`].
    ///
    /// # Errors
    /// Returns an error if the type could not be parsed
    /// from the given [`NbtCompound`].
    fn from_compound(nbt: &NbtCompound) -> Result<Self, ConvertError>;

    /// Convert the type into [`NbtCompound`].
    ///
    /// # Errors
    /// Returns an error if the type could not be converted
    /// into an [`NbtCompound`].
    fn into_compound(&self) -> Result<NbtCompound, ConvertError>;

    /// Convert the type into [`UnnamedNbt`].
    ///
    /// # Errors
    /// Returns an error if the type could not be converted
    /// into an [`UnnamedNbt`].
    #[inline]
    fn into_nbt(&self) -> Result<UnnamedNbt, ConvertError> {
        self.into_compound().map(UnnamedNbt::new)
    }
}

/// An error that can occur when converting between a type and NBT.
#[derive(Debug, thiserror::Error)]
pub enum ConvertError {}
