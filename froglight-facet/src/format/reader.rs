//! TODO

use alloc::boxed::Box;
use core::error::Error;

use facet::ReflectError;

/// A `no_std`-compatible reader.
pub struct Reader<'a> {
    position: usize,
    remaining: &'a [u8],
}

impl<'a> Reader<'a> {
    /// Create a new [`Reader`] for the given slice.
    #[inline]
    #[must_use]
    pub const fn new(slice: &'a [u8]) -> Self { Self { position: 0, remaining: slice } }

    /// Get the "position" of the reader,
    /// or the number of bytes read so far.
    #[inline]
    #[must_use]
    pub const fn position(&self) -> usize { self.position }

    /// Get the remaining bytes in the reader.
    #[inline]
    #[must_use]
    pub const fn remaining(&self) -> &'a [u8] { self.remaining }

    /// Get the next `len` bytes from the reader.
    ///
    /// # Errors
    ///
    /// Returns an error if the reader doesn't have enough bytes remaining.
    pub fn get(&mut self, len: usize) -> Result<&'a [u8], ReaderError> {
        if self.remaining.len() < len {
            Err(ReaderError::EndOfInput(len - self.remaining.len()))
        } else {
            let (result, remaining) = self.remaining.split_at(len);
            self.position += len;
            self.remaining = remaining;
            Ok(result)
        }
    }

    /// Get the next `N` bytes from the reader.
    ///
    /// # Errors
    ///
    /// Returns an error if the reader doesn't have enough bytes remaining.
    pub fn get_array<const N: usize>(&mut self) -> Result<&'a [u8; N], ReaderError> {
        if let Some((result, remaining)) = self.remaining.split_first_chunk::<N>() {
            self.position += N;
            self.remaining = remaining;
            Ok(result)
        } else {
            Err(ReaderError::EndOfInput(N - self.remaining.len()))
        }
    }

    /// Consume the next `length` bytes from the reader without returning them.
    ///
    /// # Errors
    ///
    /// Returns an error if the reader doesn't have enough bytes remaining.
    pub const fn consume(&mut self, length: usize) -> Result<(), ReaderError> {
        if let Some((_, rem)) = self.remaining.split_at_checked(length) {
            self.position += length;
            self.remaining = rem;
            Ok(())
        } else {
            Err(ReaderError::EndOfInput(length - self.remaining.len()))
        }
    }

    /// Reborrow the [`Reader`] with a shorter lifetime.
    #[inline]
    #[must_use]
    pub const fn reborrow(&mut self) -> Reader<'_> {
        Reader { position: self.position, remaining: self.remaining }
    }
}

// -------------------------------------------------------------------------------------------------

/// Errors that can occur while using a [`Reader`].
#[derive(Debug)]
pub enum ReaderError {
    /// The reader encountered an invalid boolean.
    InvalidBool(u8),
    /// The [`Reader`] reached the end but expected more data.
    EndOfInput(usize),
    /// An error occurred during reflection.
    Reflect(ReflectError),

    /// An I/O error occurred while reading.
    #[cfg(feature = "std")]
    IO(std::io::Error),
    /// Some other type of error occurred.
    Other(Box<dyn Error + Send + Sync>),
}

impl ReaderError {
    /// Create a [`ReaderError::Other`].
    #[inline]
    #[must_use]
    pub fn other<T: Error + Send + Sync + 'static>(err: T) -> Self { Self::Other(Box::new(err)) }
}

impl From<ReflectError> for ReaderError {
    #[inline]
    fn from(err: ReflectError) -> Self { Self::Reflect(err) }
}
#[cfg(feature = "std")]
impl From<std::io::Error> for ReaderError {
    #[inline]
    fn from(err: std::io::Error) -> Self { Self::IO(err) }
}
