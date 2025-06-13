#[cfg(feature = "bevy")]
use bevy_reflect::prelude::*;
use derive_more::{AsMut, AsRef, Deref, DerefMut, From, Into};
#[cfg(feature = "io")]
use froglight_common::prelude::*;
#[cfg(feature = "io")]
use froglight_io::{
    prelude::*,
    version::{FrogReadVersion, FrogWriteVersion},
};
use smallvec::{Array, SmallVec};

/// A wrapper around a [`Vec<T>`] that reads
/// all remaining bytes as a vector of `T`s.
///
/// Must be used as the last field in a packet.
#[repr(transparent)]
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, Deref, DerefMut, AsRef, AsMut, From, Into)]
#[cfg_attr(feature = "bevy", derive(Reflect))]
pub struct UnsizedVec<T>(Vec<T>);

impl<T> UnsizedVec<T> {
    /// Create a new, empty [`UnsizedVec`].
    #[must_use]
    pub const fn new() -> Self { Self(Vec::new()) }

    /// Create a new [`UnsizedVec`] with at least the specified capacity.
    #[must_use]
    pub fn with_capacity(capacity: usize) -> Self { Self(Vec::with_capacity(capacity)) }

    /// Create a new [`UnsizedVec`] from an existing [`Vec<T>`].
    #[must_use]
    pub const fn from_vec(vec: Vec<T>) -> Self { Self(vec) }

    /// Return the inner [`Vec<T>`] of this [`UnsizedVec`].
    #[must_use]
    pub fn into_inner(self) -> Vec<T> { self.0 }
}

// -------------------------------------------------------------------------------------------------

#[cfg(feature = "io")]
impl<V: Version, T: FrogReadVersion<V>> FrogReadVersion<V> for UnsizedVec<T> {
    fn frog_read(buffer: &mut impl std::io::Read) -> Result<Self, ReadError> {
        #[cfg(feature = "trace")]
        tracing::trace!(target: "froglight_io::read", "Reading struct \"UnsizedVec<{}>\"", core::any::type_name::<T>());

        let mut buf = Vec::new();
        loop {
            match T::frog_read(buffer) {
                Ok(item) => buf.push(item),
                Err(ReadError::Io(err)) if err.kind() == std::io::ErrorKind::UnexpectedEof => break,
                Err(err) => return Err(err),
            }
        }
        Ok(Self(buf))
    }
}
#[cfg(feature = "io")]
impl<V: Version, T: FrogWriteVersion<V>> FrogWriteVersion<V> for UnsizedVec<T> {
    fn frog_write(&self, buffer: &mut impl std::io::Write) -> Result<usize, WriteError> {
        #[cfg(feature = "trace")]
        tracing::trace!(target: "froglight_io::write", "Writing struct \"UnsizedVec<{}>\"", core::any::type_name::<T>());
        self.0.iter().try_fold(0usize, |acc, item| item.frog_write(buffer).map(|size| acc + size))
    }

    #[inline]
    fn frog_len(&self) -> usize { self.0.iter().map(|item| item.frog_len()).sum() }
}

// -------------------------------------------------------------------------------------------------

/// A wrapper around a [`SmallVec<A>`] that stores
/// all remaining bytes from the buffer.
///
/// Must be used as the last field in a packet.
#[repr(transparent)]
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, Deref, DerefMut, AsRef, AsMut, From, Into)]
#[cfg_attr(feature = "bevy", derive(Reflect))]
pub struct UnsizedBuffer<A: Array<Item = u8>>(SmallVec<A>);

impl<A: Array<Item = u8>> UnsizedBuffer<A> {
    /// Create a new, empty [`UnsizedBuffer`].
    #[must_use]
    pub fn new() -> Self { Self(SmallVec::new()) }

    /// Create a new [`UnsizedBuffer`] with at least the specified capacity.
    #[must_use]
    pub fn with_capacity(capacity: usize) -> Self { Self(SmallVec::with_capacity(capacity)) }

    /// Create a new [`UnsizedBuffer`] from an existing [`SmallVec<A>`].
    #[must_use]
    pub const fn from_smallvec(smallvec: SmallVec<A>) -> Self { Self(smallvec) }

    /// Return the inner [`SmallVec<A>`] of this [`UnsizedBuffer`].
    #[must_use]
    pub fn into_inner(self) -> SmallVec<A> { self.0 }

    /// Create a new [`UnsizedBuffer`] from an existing [`Vec<u8>`].
    #[must_use]
    pub fn from_vec(vec: Vec<u8>) -> Self { Self(SmallVec::from_vec(vec)) }

    /// Return the inner [`SmallVec<A>`] as a [`Vec<A>`].
    #[must_use]
    pub fn into_vec(self) -> Vec<u8> { self.0.into_vec() }
}

impl<const N: usize> UnsizedBuffer<[u8; N]> {
    /// Create a new, empty [`UnsizedBuffer`].
    #[must_use]
    pub const fn new_const() -> Self { Self(SmallVec::new_const()) }

    /// Create a new [`UnsizedBuffer`] from an existing array of bytes.
    #[must_use]
    pub const fn from_const(buf: [u8; N]) -> Self { Self(SmallVec::from_const(buf)) }
}

// -------------------------------------------------------------------------------------------------

#[cfg(feature = "io")]
impl<A: Array<Item = u8>> FrogRead for UnsizedBuffer<A> {
    fn frog_read(buffer: &mut impl std::io::Read) -> Result<Self, ReadError> {
        #[cfg(feature = "trace")]
        tracing::trace!(target: "froglight_io::read", "Reading struct \"UnsizedBuffer<{}>\"", core::any::type_name::<A>());
        let mut buf = Vec::new();
        buffer.read_to_end(&mut buf)?;
        Ok(Self(SmallVec::from_vec(buf)))
    }
}

#[cfg(feature = "io")]
impl<A: Array<Item = u8>> FrogWrite for UnsizedBuffer<A> {
    fn frog_write(&self, buffer: &mut impl std::io::Write) -> Result<usize, WriteError> {
        #[cfg(feature = "trace")]
        tracing::trace!(target: "froglight_io::write", "Writing struct \"UnsizedBuffer<{}>\"", core::any::type_name::<A>());
        buffer.write_all(&self.0)?;
        Ok(self.0.len())
    }

    #[inline]
    fn frog_len(&self) -> usize { self.0.len() }
}
