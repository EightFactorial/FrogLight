//! Traits and implementations for encoding and decoding types.

use std::io::{Read, Write};

mod froglight_common_impl;
mod smol_str_impl;
mod std_impl;

#[cfg(feature = "glam")]
mod glam_impl;
#[cfg(feature = "hashbrown")]
mod hashbrown_impl;
#[cfg(feature = "smallvec")]
mod smallvec_impl;
#[cfg(feature = "uuid")]
mod uuid_impl;

/// A trait for reading data from a buffer.
pub trait FrogRead: Sized {
    /// Read the data from the given buffer.
    ///
    /// # Errors
    /// Returns an error if the type does not match the buffer,
    /// or if the buffer fails to be read from.
    fn frog_read(buffer: &mut impl Read) -> Result<Self, ReadError>;
}

/// A trait for writing data to a buffer.
pub trait FrogWrite {
    /// Write the data to the given buffer.
    ///
    /// # Errors
    /// Returns an error if the buffer fails be written to.
    fn frog_write(&self, buffer: &mut impl Write) -> Result<usize, WriteError>;

    /// Return the length of the encoded data.
    fn frog_len(&self) -> usize;

    /// Create a buffer containing the encoded data.
    ///
    /// # Errors
    /// Returns an error if the buffer fails be written to.
    fn frog_to_buf<B: Default + Write>(&self) -> Result<B, WriteError> {
        let mut buf = B::default();
        self.frog_write(&mut buf).map(|_| buf)
    }
}

/// Errors that can occur when reading from a buffer.
#[derive(Debug, thiserror::Error)]
pub enum ReadError {
    /// An error that occurred while reading from a buffer.
    #[error("Failed to read from buffer: {0}")]
    Io(#[from] std::io::Error),
    /// An error that occurred while reading a string value.
    #[error("Failed to parse UTF-8: {0}")]
    Utf8(#[from] std::string::FromUtf8Error),

    /// An error that occurred while reading a bool value.
    #[error("Invalid bool value: {0}")]
    InvalidBool(u8),
    /// An error that occurred while reading an enum value.
    #[error("Invalid enum variant for \"{0}\": {1}")]
    InvalidEnum(&'static str, u32),

    /// An error that occurred while parsing a value from JSON.
    #[cfg(feature = "serde")]
    #[error("Failed to parse from JSON: {0}")]
    Json(#[from] serde_json::Error),
}

/// Errors that can occur when writing to a buffer.
#[derive(Debug, thiserror::Error)]
pub enum WriteError {
    /// An error that occurred while writing to a buffer.
    #[error("Failed to write to buffer: {0}")]
    Io(#[from] std::io::Error),

    /// An error that occurred while parsing a value into JSON.
    #[cfg(feature = "serde")]
    #[error("Failed to parse into JSON: {0}")]
    Json(#[from] serde_json::Error),
}
