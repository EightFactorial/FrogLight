//! Traits and implementations for variable-length encoding and decoding types.

use std::io::{Read, Write};

use crate::standard::{ReadError, WriteError};

mod froglight_impl;
mod std_impl;

#[cfg(feature = "bevy_platform")]
mod bevy_impl;
#[cfg(feature = "glam")]
mod glam_impl;
#[cfg(feature = "hashbrown")]
mod hashbrown_impl;
#[cfg(feature = "smallvec")]
mod smallvec_impl;

/// A trait for reading data from a buffer.
pub trait FrogVarRead: Sized {
    /// Read the data from the given buffer.
    ///
    /// # Errors
    /// Returns an error if the type does not match the buffer,
    /// or if the buffer fails to be read from.
    fn frog_var_read(buffer: &mut impl Read) -> Result<Self, ReadError>;
}

/// A trait for writing data to a buffer.
pub trait FrogVarWrite {
    /// Write the data to the given buffer.
    ///
    /// # Errors
    /// Returns an error if the buffer fails be written to.
    fn frog_var_write(&self, buffer: &mut impl Write) -> Result<usize, WriteError>;

    /// Return the length of the variably-encoded data.
    fn frog_var_len(&self) -> usize;

    /// Create a buffer containing the encoded data.
    ///
    /// # Errors
    /// Returns an error if the buffer fails be written to.
    fn frog_to_var_buf<B: Default + Write>(&self) -> Result<B, WriteError> {
        let mut buf = B::default();
        self.frog_var_write(&mut buf).map(|_| buf)
    }
}
