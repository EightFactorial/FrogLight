//! TODO

use crate::nbt::{NbtCompound, UnnamedNbt};

/// A trait for converting between a type and NBT.
pub trait ConvertNbt: Sized {
    /// Parse the type from [`NbtCompound`].
    fn from_compound(nbt: NbtCompound) -> Self;

    /// Convert the type into [`NbtCompound`].
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
