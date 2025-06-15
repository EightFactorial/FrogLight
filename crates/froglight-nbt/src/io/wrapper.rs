#[cfg(feature = "io")]
use std::io::{Read, Write};

#[cfg(feature = "reflect")]
use bevy_reflect::prelude::*;
use derive_more::{AsMut, AsRef, Deref, DerefMut};
#[cfg(feature = "io")]
use froglight_io::prelude::*;

use crate::prelude::*;

/// A wrapper around any type that implements [`FromTag`] and [`IntoTag`].
///
/// Allows reading and writing NBT data using
/// the [`FrogRead`] and [`FrogWrite`] traits.
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Deref, DerefMut, AsRef, AsMut)]
#[cfg_attr(feature = "reflect", derive(Reflect))]
pub struct NbtWrapper<T: FromTag + IntoTag + MaybeReflect>(pub T);

impl<T: FromTag + IntoTag + MaybeReflect> NbtWrapper<T> {
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

#[cfg(feature = "io")]
impl<T: FromTag + IntoTag + MaybeReflect> FrogRead for NbtWrapper<T> {
    fn frog_read(buffer: &mut impl Read) -> Result<Self, ReadError> {
        T::from_tag(&NbtTag::frog_read(buffer)?)
            .map_or_else(|err| todo!("{err}"), |val| Ok(Self(val)))
    }
}

#[cfg(feature = "io")]
impl<T: FromTag + IntoTag + MaybeReflect> FrogWrite for NbtWrapper<T> {
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
pub struct NbtCompoundWrapper<T: FromCompound + IntoCompound>(pub T);

impl<T: FromCompound + IntoCompound> NbtCompoundWrapper<T> {
    /// Wraps the given value in a [`NbtCompoundWrapper`].
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

    /// Unwraps the value from the [`NbtCompoundWrapper`].
    #[inline]
    #[must_use]
    pub fn into_inner(self) -> T { self.0 }
}

// -------------------------------------------------------------------------------------------------

#[cfg(feature = "io")]
impl<T: FromCompound + IntoCompound> FrogRead for NbtCompoundWrapper<T> {
    fn frog_read(buffer: &mut impl Read) -> Result<Self, ReadError> {
        T::from_compound(&NbtCompound::frog_read(buffer)?)
            .map_or_else(|err| todo!("{err}"), |val| Ok(Self(val)))
    }
}

#[cfg(feature = "io")]
impl<T: FromCompound + IntoCompound> FrogWrite for NbtCompoundWrapper<T> {
    fn frog_write(&self, buffer: &mut impl Write) -> Result<usize, WriteError> {
        T::into_compound(self).map_err(|err| -> WriteError { todo!("{err}") })?.frog_write(buffer)
    }

    fn frog_len(&self) -> usize {
        T::into_compound(self).map_or_else(|err| panic!("{err}"), |val| val.frog_len())
    }
}

// -------------------------------------------------------------------------------------------------

use sealed::MaybeReflect;
mod sealed {
    #[cfg(feature = "reflect")]
    use bevy_reflect::prelude::*;

    #[cfg(feature = "reflect")]
    pub trait MaybeReflect: Reflect {}
    #[cfg(feature = "reflect")]
    impl<T: Reflect> MaybeReflect for T {}

    #[cfg(not(feature = "reflect"))]
    pub trait MaybeReflect {}
    #[cfg(not(feature = "reflect"))]
    impl<T> MaybeReflect for T {}
}
