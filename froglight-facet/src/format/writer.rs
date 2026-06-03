//! TODO

use alloc::boxed::Box;
use core::error::Error;

/// A `no_std`-compatible writer.
pub struct Writer<'a> {
    written: usize,
    inner: &'a mut dyn WriterType,
}

impl<'a> Writer<'a> {
    /// Create a new [`Writer`] for the given slice.
    #[inline]
    #[must_use]
    pub const fn new<T: WriterType>(inner: &'a mut T) -> Self { Self { written: 0, inner } }

    /// Get the total number of bytes written so far.
    #[inline]
    #[must_use]
    pub const fn total_written(&self) -> usize { self.written }

    /// Write a single byte to the writer.
    ///
    /// # Errors
    ///
    /// Returns an error if the writer cannot be written to.
    pub fn write_byte(&mut self, byte: u8) -> Result<(), WriterError> {
        self.inner.write_byte(byte)?;
        self.written += 1;
        Ok(())
    }

    /// Write a slice of bytes to the writer.
    ///
    /// # Errors
    ///
    /// Returns an error if the writer cannot be written to.
    pub fn write_bytes(&mut self, bytes: &[u8]) -> Result<(), WriterError> {
        self.written += self.inner.write_bytes(bytes)?;
        Ok(())
    }
}

impl<'a, T: WriterType> From<&'a mut T> for Writer<'a> {
    #[inline]
    fn from(inner: &'a mut T) -> Self { Self::new(inner) }
}

// -------------------------------------------------------------------------------------------------

/// A trait for types that can be written to using a [`Writer`].
pub trait WriterType {
    /// Write a single byte to the writer.
    ///
    /// # Errors
    ///
    /// Returns an error if the writer cannot be written to.
    fn write_byte(&mut self, byte: u8) -> Result<(), WriterError>;

    /// Write a slice of bytes to the writer.
    ///
    /// # Errors
    ///
    /// Returns an error if the writer cannot be written to.
    fn write_bytes(&mut self, bytes: &[u8]) -> Result<usize, WriterError>;
}

impl WriterType for [u8] {
    fn write_byte(&mut self, byte: u8) -> Result<(), WriterError> {
        if let Some(pos) = self.get_mut(0) {
            *pos = byte;
            Ok(())
        } else {
            Err(WriterError::WriterFull)
        }
    }

    fn write_bytes(&mut self, bytes: &[u8]) -> Result<usize, WriterError> {
        if let Some(pos) = self.get_mut(0..bytes.len()) {
            pos.copy_from_slice(bytes);
            Ok(bytes.len())
        } else {
            Err(WriterError::WriterFull)
        }
    }
}

#[cfg(not(feature = "std"))]
impl WriterType for alloc::vec::Vec<u8> {
    fn write_byte(&mut self, byte: u8) -> Result<(), WriterError> {
        self.push(byte);
        Ok(())
    }

    fn write_bytes(&mut self, bytes: &[u8]) -> Result<usize, WriterError> {
        self.extend_from_slice(bytes);
        Ok(bytes.len())
    }
}

#[cfg(feature = "std")]
impl<T: std::io::Write> WriterType for T {
    fn write_byte(&mut self, byte: u8) -> Result<(), WriterError> {
        std::io::Write::write(self, core::array::from_ref(&byte))
            .map_or_else(|err| Err(WriterError::IO(err)), |_| Ok(()))
    }

    fn write_bytes(&mut self, bytes: &[u8]) -> Result<usize, WriterError> {
        std::io::Write::write(self, bytes).map_err(WriterError::IO)
    }
}

// -------------------------------------------------------------------------------------------------

/// Errors that can occur while using a [`Writer`].
#[derive(Debug)]
pub enum WriterError {
    /// The writer is full and cannot accept more bytes.
    WriterFull,

    /// An I/O error occurred while writing.
    #[cfg(feature = "std")]
    IO(std::io::Error),
    /// Some other type of error occurred.
    Other(Box<dyn Error + Send + Sync>),
}

impl WriterError {
    /// Create a [`WriterError::Other`].
    #[inline]
    #[must_use]
    pub fn other<T: Error + Send + Sync + 'static>(err: T) -> Self { Self::Other(Box::new(err)) }
}

#[cfg(feature = "std")]
impl From<std::io::Error> for WriterError {
    #[inline]
    fn from(err: std::io::Error) -> Self { Self::IO(err) }
}
