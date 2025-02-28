//! TODO

use crate::nbt::{NbtCompound, UnnamedNbt};

/// A trait for converting between a type and NBT.
pub trait ConvertNbt: Sized {
    /// Parse the type from [`NbtCompound`].
    ///
    /// # Errors
    /// Returns an error if the type could not be parsed
    /// from the given [`NbtCompound`].
    fn from_compound(nbt: NbtCompound) -> Result<Self, ConvertError>;

    /// Convert the type into [`NbtCompound`].
    #[must_use]
    fn into_compound(self) -> NbtCompound;

    /// Convert the type into [`UnnamedNbt`].
    #[inline]
    #[must_use]
    fn into_nbt(self) -> UnnamedNbt { UnnamedNbt::new(self.into_compound()) }
}

impl<T: ConvertNbt> From<T> for NbtCompound {
    #[inline]
    fn from(value: T) -> Self { value.into_compound() }
}

/// An error that can occur when converting between a type and NBT.
#[derive(Debug, thiserror::Error)]
pub enum ConvertError {}
