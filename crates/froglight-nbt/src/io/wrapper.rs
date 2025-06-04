use std::io::{Read, Write};

use derive_more::{AsMut, AsRef, Deref, DerefMut};
use froglight_io::prelude::*;

use crate::{
    convert::{FromCompound, FromTag, IntoCompound, IntoTag},
    nbt::{NbtCompound, NbtTag},
};

/// A wrapper around any type that implements [`FromTag`] and [`IntoTag`].
///
/// Allows reading and writing NBT data using
/// the [`FrogRead`] and [`FrogWrite`] traits.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Deref, DerefMut, AsRef, AsMut)]
pub struct NbtWrapper<T: FromTag + IntoTag>(pub T);

impl<T: FromTag + IntoTag> NbtWrapper<T> {
    /// Wraps the given value in a [`NbtWrapper`].
    #[inline]
    #[must_use]
    pub const fn new(value: T) -> Self { Self(value) }

    /// Get a reference to the inner value.
    #[inline]
    #[must_use]
    pub const fn inner(&self) -> &T { &self.0 }

    /// Get a mutable reference to the inner value.
    #[inline]
    #[must_use]
    pub const fn inner_mut(&mut self) -> &mut T { &mut self.0 }

    /// Unwraps the value from the [`NbtWrapper`].
    #[inline]
    #[must_use]
    pub fn into_inner(self) -> T { self.0 }
}

// -------------------------------------------------------------------------------------------------

impl<T: FromTag + IntoTag> FrogRead for NbtWrapper<T> {
    fn frog_read(buffer: &mut impl Read) -> Result<Self, ReadError> {
        T::from_tag(&NbtTag::frog_read(buffer)?)
            .map_or_else(|err| todo!("{err}"), |val| Ok(Self(val)))
    }
}

impl<T: FromTag + IntoTag> FrogWrite for NbtWrapper<T> {
    fn frog_write(&self, buffer: &mut impl Write) -> Result<usize, WriteError> {
        T::into_tag(self).map_err(|err| -> WriteError { todo!("{err}") })?.frog_write(buffer)
    }

    fn frog_len(&self) -> usize {
        T::into_tag(self).map_or_else(|err| panic!("{err}"), |val| val.frog_len())
    }
}

// -------------------------------------------------------------------------------------------------

/// A wrapper around any type that implements [`FromCompound`] and
/// [`IntoCompound`].
///
/// Allows reading and writing NBT data using
/// the [`FrogRead`] and [`FrogWrite`] traits.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Deref, DerefMut, AsRef, AsMut)]
pub struct NbtTagWrapper<T: FromCompound + IntoCompound>(pub T);

impl<T: FromCompound + IntoCompound> NbtTagWrapper<T> {
    /// Wraps the given value in a [`NbtTagWrapper`].
    #[inline]
    #[must_use]
    pub const fn new(value: T) -> Self { Self(value) }

    /// Get a reference to the inner value.
    #[inline]
    #[must_use]
    pub const fn inner(&self) -> &T { &self.0 }

    /// Get a mutable reference to the inner value.
    #[inline]
    #[must_use]
    pub const fn inner_mut(&mut self) -> &mut T { &mut self.0 }

    /// Unwraps the value from the [`NbtTagWrapper`].
    #[inline]
    #[must_use]
    pub fn into_inner(self) -> T { self.0 }
}

// -------------------------------------------------------------------------------------------------

impl<T: FromCompound + IntoCompound> FrogRead for NbtTagWrapper<T> {
    fn frog_read(buffer: &mut impl Read) -> Result<Self, ReadError> {
        T::from_compound(&NbtCompound::frog_read(buffer)?)
            .map_or_else(|err| todo!("{err}"), |val| Ok(Self(val)))
    }
}

impl<T: FromCompound + IntoCompound> FrogWrite for NbtTagWrapper<T> {
    fn frog_write(&self, buffer: &mut impl Write) -> Result<usize, WriteError> {
        T::into_compound(self).map_err(|err| -> WriteError { todo!("{err}") })?.frog_write(buffer)
    }

    fn frog_len(&self) -> usize {
        T::into_compound(self).map_or_else(|err| panic!("{err}"), |val| val.frog_len())
    }
}
