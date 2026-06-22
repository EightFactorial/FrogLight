//! TODO

use alloc::boxed::Box;
use core::error::Error;

use facet::ReflectError;

/// A `no_std`-compatible writer.
pub struct Writer<'a> {
    position: usize,
    inner: &'a mut dyn WriterType,
}

impl<'a> Writer<'a> {
    /// Create a new [`Writer`] for the given slice.
    #[inline]
    #[must_use]
    pub const fn new<T: WriterType>(inner: &'a mut T) -> Self { Self { position: 0, inner } }

    /// Get the "position" of the writer,
    /// or the number of bytes written so far.
    #[inline]
    #[must_use]
    pub const fn position(&self) -> usize { self.position }

    /// Write a single byte to the writer.
    ///
    /// # Errors
    ///
    /// Returns an error if the writer cannot be written to.
    pub fn write_byte(&mut self, byte: u8) -> Result<(), WriterError> {
        self.inner.write_byte(self.position, byte)?;
        self.position += 1;
        Ok(())
    }

    /// Write a slice of bytes to the writer.
    ///
    /// # Errors
    ///
    /// Returns an error if the writer cannot be written to.
    pub fn write_bytes(&mut self, bytes: &[u8]) -> Result<(), WriterError> {
        self.inner.write_bytes(self.position, bytes)?;
        self.position += bytes.len();
        Ok(())
    }

    /// Reborrow the [`Writer`] with a shorter lifetime.
    #[inline]
    #[must_use]
    pub const fn reborrow(&mut self) -> Writer<'_> {
        Writer { position: self.position, inner: self.inner }
    }
}

impl WriterType for Writer<'_> {
    #[inline]
    fn write_byte(&mut self, position: usize, byte: u8) -> Result<(), WriterError> {
        self.inner.write_byte(position, byte)
    }

    #[inline]
    fn write_bytes(&mut self, position: usize, bytes: &[u8]) -> Result<(), WriterError> {
        self.inner.write_bytes(position, bytes)
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
    fn write_byte(&mut self, position: usize, byte: u8) -> Result<(), WriterError>;

    /// Write a slice of bytes to the writer.
    ///
    /// # Errors
    ///
    /// Returns an error if the writer cannot be written to.
    fn write_bytes(&mut self, position: usize, bytes: &[u8]) -> Result<(), WriterError>;
}

impl WriterType for [u8] {
    fn write_byte(&mut self, position: usize, byte: u8) -> Result<(), WriterError> {
        if let Some(pos) = self.get_mut(position) {
            *pos = byte;
            Ok(())
        } else {
            Err(WriterError::WriterFull)
        }
    }

    fn write_bytes(&mut self, position: usize, bytes: &[u8]) -> Result<(), WriterError> {
        if let Some(pos) = self.get_mut(position..position + bytes.len()) {
            pos.copy_from_slice(bytes);
            Ok(())
        } else {
            Err(WriterError::WriterFull)
        }
    }
}

#[cfg(not(feature = "std"))]
impl WriterType for alloc::vec::Vec<u8> {
    fn write_byte(&mut self, _: usize, byte: u8) -> Result<(), WriterError> {
        self.push(byte);
        Ok(())
    }

    fn write_bytes(&mut self, _: usize, bytes: &[u8]) -> Result<(), WriterError> {
        self.extend_from_slice(bytes);
        Ok(())
    }
}

#[cfg(feature = "std")]
impl<T: std::io::Write> WriterType for T {
    fn write_byte(&mut self, _: usize, byte: u8) -> Result<(), WriterError> {
        std::io::Write::write_all(self, core::array::from_ref(&byte)).map_err(WriterError::IO)
    }

    fn write_bytes(&mut self, _: usize, bytes: &[u8]) -> Result<(), WriterError> {
        std::io::Write::write_all(self, bytes).map_err(WriterError::IO)
    }
}

// -------------------------------------------------------------------------------------------------

/// Errors that can occur while using a [`Writer`].
#[derive(Debug)]
pub enum WriterError {
    /// The writer is full and cannot accept more bytes.
    WriterFull,
    /// An error occurred during reflection.
    Reflect(ReflectError),

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

    /// Create a [`WriterError::Other`] from a string.
    #[inline]
    #[must_use]
    pub fn from_string(err: alloc::string::String) -> Self {
        Self::Other(Box::<dyn Error + Send + Sync>::from(err))
    }
}

impl From<ReflectError> for WriterError {
    #[inline]
    fn from(err: ReflectError) -> Self { Self::Reflect(err) }
}
#[cfg(feature = "std")]
impl From<std::io::Error> for WriterError {
    #[inline]
    fn from(err: std::io::Error) -> Self { Self::IO(err) }
}
