#[cfg(feature = "bevy")]
use bevy_reflect::prelude::*;
use derive_more::{From, Into};

use crate::{
    mutf8::{Mutf8Str, Mutf8String},
    nbt::NbtCompound,
};

/// A named set of NBT tags.
#[repr(transparent)]
#[derive(Debug, PartialEq, From, Into)]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(Debug, PartialEq))]
pub struct NamedNbt(Option<(Mutf8String, NbtCompound)>);

impl NamedNbt {
    /// Create a new [`NamedNbt`] from a name and [`NbtCompound`].
    #[inline]
    #[must_use]
    pub const fn new(name: Mutf8String, compound: NbtCompound) -> Self {
        Self(Some((name, compound)))
    }

    /// Create a new empty [`NamedNbt`].
    #[inline]
    #[must_use]
    pub const fn new_empty() -> Self { Self(None) }

    /// Read a [`NamedNbt`] from a reader.
    ///
    /// # Errors
    /// Returns a [`froglight_io::standard::ReadError`] if the read fails.
    #[inline]
    #[cfg(feature = "io")]
    pub fn read_from(
        reader: &mut impl std::io::Read,
    ) -> Result<Self, froglight_io::standard::ReadError> {
        froglight_io::standard::FrogRead::frog_read(reader)
    }

    /// Write a [`NamedNbt`] to a writer, returning the number of bytes written.
    ///
    /// # Errors
    /// Returns a [`froglight_io::standard::WriteError`] if the write fails.
    #[inline]
    #[cfg(feature = "io")]
    pub fn write_to(
        &self,
        writer: &mut impl std::io::Write,
    ) -> Result<usize, froglight_io::standard::WriteError> {
        froglight_io::standard::FrogWrite::frog_write(self, writer)
    }

    /// Get the name of the [`NamedNbt`].
    #[inline]
    #[must_use]
    pub fn name(&self) -> Option<&Mutf8Str> { self.0.as_ref().map(|(a, _)| a.as_mutf8_str()) }

    /// Get the name of the [`NamedNbt`] mutably.
    #[inline]
    #[must_use]
    pub fn name_mut(&mut self) -> Option<&mut Mutf8String> { self.0.as_mut().map(|(a, _)| a) }

    /// Get the [`NbtCompound`] of the [`NamedNbt`].
    #[inline]
    #[must_use]
    pub fn compound(&self) -> Option<&NbtCompound> { self.0.as_ref().map(|(_, b)| b) }

    /// Get the [`NbtCompound`] of the [`NamedNbt`] mutably.
    #[inline]
    #[must_use]
    pub fn compound_mut(&mut self) -> Option<&mut NbtCompound> { self.0.as_mut().map(|(_, b)| b) }

    /// Create an [`UnnamedNbt`] from this [`NamedNbt`].
    #[inline]
    #[must_use]
    pub fn into_unnamed(self) -> UnnamedNbt {
        match self.0 {
            Some((_, compound)) => UnnamedNbt::new(compound),
            None => UnnamedNbt::new_empty(),
        }
    }
}

// -------------------------------------------------------------------------------------------------

/// An unnamed set of NBT tags.
#[repr(transparent)]
#[derive(Debug, Default, PartialEq, From, Into)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize), serde(transparent))]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(Debug, Default, PartialEq))]
pub struct UnnamedNbt(Option<NbtCompound>);

impl UnnamedNbt {
    /// Create a new [`UnnamedNbt`] from a [`NbtCompound`].
    #[inline]
    #[must_use]
    pub const fn new(compound: NbtCompound) -> Self { Self(Some(compound)) }

    /// Create a new empty [`UnnamedNbt`].
    #[inline]
    #[must_use]
    pub const fn new_empty() -> Self { Self(None) }

    /// Read an [`UnnamedNbt`] from a reader.
    ///
    /// # Errors
    /// Returns a [`froglight_io::standard::ReadError`] if the read fails.
    #[inline]
    #[cfg(feature = "io")]
    pub fn read_from(
        reader: &mut impl std::io::Read,
    ) -> Result<Self, froglight_io::standard::ReadError> {
        froglight_io::standard::FrogRead::frog_read(reader)
    }

    /// Write an [`UnnamedNbt`] to a writer, returning the number of bytes
    /// written.
    ///
    /// # Errors
    /// Returns a [`froglight_io::standard::WriteError`] if the write fails.
    #[inline]
    #[cfg(feature = "io")]
    pub fn write_to(
        &self,
        writer: &mut impl std::io::Write,
    ) -> Result<usize, froglight_io::standard::WriteError> {
        froglight_io::standard::FrogWrite::frog_write(self, writer)
    }

    /// Get the [`NbtCompound`] of the [`UnnamedNbt`].
    #[inline]
    #[must_use]
    pub fn compound(&self) -> Option<&NbtCompound> { self.0.as_ref() }

    /// Get the [`NbtCompound`] of the [`UnnamedNbt`] mutably.
    #[inline]
    #[must_use]
    pub fn compound_mut(&mut self) -> Option<&mut NbtCompound> { self.0.as_mut() }

    /// Get the inner [`NbtCompound`] of the [`UnnamedNbt`].
    #[inline]
    #[must_use]
    pub fn into_inner(self) -> Option<NbtCompound> { self.0 }
}

impl AsRef<Option<NbtCompound>> for UnnamedNbt {
    fn as_ref(&self) -> &Option<NbtCompound> { &self.0 }
}
impl AsMut<Option<NbtCompound>> for UnnamedNbt {
    fn as_mut(&mut self) -> &mut Option<NbtCompound> { &mut self.0 }
}

impl std::ops::Deref for UnnamedNbt {
    type Target = Option<NbtCompound>;

    fn deref(&self) -> &Self::Target { &self.0 }
}
impl std::ops::DerefMut for UnnamedNbt {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
}

impl From<NamedNbt> for UnnamedNbt {
    fn from(named: NamedNbt) -> Self { named.into_unnamed() }
}
impl<T: Into<NbtCompound>> From<T> for UnnamedNbt {
    fn from(compound: T) -> Self { Self::new(compound.into()) }
}

impl<T> FromIterator<T> for UnnamedNbt
where NbtCompound: FromIterator<T>
{
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Self::new(iter.into_iter().collect())
    }
}
