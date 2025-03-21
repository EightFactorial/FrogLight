//! Traits for reading versioned data to and from a buffer.

use std::io::{Read, Write};

use froglight_common::version::Version;

use crate::prelude::*;

/// A trait for reading versioned data from a buffer.
pub trait FrogReadVersion<V: Version>: Sized {
    /// Read the versioned data from the given buffer.
    ///
    /// # Errors
    /// Returns an error if the type does not match the buffer,
    /// or if the buffer fails to be read from.
    fn frog_read(buffer: &mut impl Read) -> Result<Self, ReadError>;
}

impl<T: FrogRead, V: Version> FrogReadVersion<V> for T {
    #[inline]
    fn frog_read(buffer: &mut impl Read) -> Result<Self, ReadError> { T::frog_read(buffer) }
}

// -------------------------------------------------------------------------------------------------

/// A trait for writing versioned data to a buffer.
pub trait FrogWriteVersion<V: Version> {
    /// Write the versioned data to the given buffer.
    ///
    /// # Errors
    /// Returns an error if the buffer fails to be written to.
    fn frog_write(&self, buffer: &mut impl Write) -> Result<usize, WriteError>;

    /// Return the length of the data if it were to be encoded.
    fn frog_len(&self) -> usize;
}

impl<T: FrogWrite, V: Version> FrogWriteVersion<V> for T {
    #[inline]
    fn frog_write(&self, buffer: &mut impl Write) -> Result<usize, WriteError> {
        T::frog_write(self, buffer)
    }

    #[inline]
    fn frog_len(&self) -> usize { T::frog_len(self) }
}
