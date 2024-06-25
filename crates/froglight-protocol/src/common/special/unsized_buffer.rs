use std::io::Read;

use derive_more::{Deref, DerefMut};

use crate::protocol::{FrogRead, FrogWrite, ReadError, WriteError};

/// A buffer that contains encoded data.
///
/// Unlike a [`Vec<u8>`], when encoded it is not prefixed with a length.
///
/// When used as a field in a packet, the buffer takes up the entire length of
/// the packet.
///
/// For this reason, it *must* be the last field in the packet.
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, Deref, DerefMut)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
pub struct UnsizedBuffer(
    #[cfg(feature = "smallvec")] smallvec::SmallVec<[u8; Self::BUFFER_SIZE]>,
    #[cfg(not(feature = "smallvec"))] Vec<u8>,
);

impl UnsizedBuffer {
    /// The default buffer size for an [`UnsizedBuffer`].
    pub const BUFFER_SIZE: usize = 16;

    /// Creates a new [`UnsizedBuffer`]
    #[must_use]
    pub const fn new() -> Self {
        #[cfg(feature = "smallvec")]
        {
            Self(smallvec::SmallVec::new_const())
        }
        #[cfg(not(feature = "smallvec"))]
        {
            Self(Vec::new())
        }
    }

    /// Creates a new [`UnsizedBuffer`] with the specified capacity.
    #[must_use]
    pub fn with_capacity(capacity: usize) -> Self {
        #[cfg(feature = "smallvec")]
        {
            Self(smallvec::SmallVec::with_capacity(capacity))
        }
        #[cfg(not(feature = "smallvec"))]
        {
            Self(Vec::with_capacity(capacity))
        }
    }

    /// Creates a new [`UnsizedBuffer`] from a vector.
    #[must_use]
    pub fn from_vec(vec: Vec<u8>) -> Self {
        #[cfg(feature = "smallvec")]
        {
            Self(smallvec::SmallVec::from_vec(vec))
        }
        #[cfg(not(feature = "smallvec"))]
        {
            Self(vec)
        }
    }

    /// Creates a new [`UnsizedBuffer`] from a slice.
    #[must_use]
    pub fn from_slice(slice: &[u8]) -> Self {
        #[cfg(feature = "smallvec")]
        {
            Self(smallvec::SmallVec::from_slice(slice))
        }
        #[cfg(not(feature = "smallvec"))]
        {
            Self(slice.to_vec())
        }
    }

    /// Creates a new [`UnsizedBuffer`] from an array.
    #[must_use]
    pub fn from_array<const N: usize>(arr: [u8; N]) -> Self {
        #[cfg(feature = "smallvec")]
        {
            let mut smallvec = smallvec::SmallVec::with_capacity(N);
            smallvec.extend(arr);
            Self(smallvec)
        }
        #[cfg(not(feature = "smallvec"))]
        {
            Self(arr.to_vec())
        }
    }

    /// Creates a new [`UnsizedBuffer`] from a const array
    #[must_use]
    #[cfg(feature = "smallvec")]
    pub const fn from_const(arr: [u8; Self::BUFFER_SIZE]) -> Self {
        Self(smallvec::SmallVec::from_const(arr))
    }
}

impl std::io::Write for UnsizedBuffer {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        std::io::Write::write(&mut self.0, buf)
    }

    fn flush(&mut self) -> std::io::Result<()> { std::io::Write::flush(&mut self.0) }
}

impl std::io::Read for UnsizedBuffer {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let len = std::cmp::min(buf.len(), self.0.len());
        buf[..len].copy_from_slice(&self.0[..len]);
        self.0.drain(..len);
        Ok(len)
    }
}

impl AsRef<[u8]> for UnsizedBuffer {
    fn as_ref(&self) -> &[u8] { &self.0 }
}

impl From<Vec<u8>> for UnsizedBuffer {
    fn from(bytes: Vec<u8>) -> Self { Self::from_vec(bytes) }
}

impl From<&[u8]> for UnsizedBuffer {
    fn from(bytes: &[u8]) -> Self { Self::from_slice(bytes) }
}

impl<const N: usize> From<[u8; N]> for UnsizedBuffer {
    fn from(value: [u8; N]) -> Self { Self::from_array(value) }
}

impl From<UnsizedBuffer> for Vec<u8> {
    fn from(buffer: UnsizedBuffer) -> Self { buffer.0.to_vec() }
}

impl FrogWrite for UnsizedBuffer {
    fn fg_write(&self, buf: &mut (impl std::io::Write + ?Sized)) -> Result<(), WriteError> {
        buf.write_all(&self.0)?;
        Ok(())
    }
}

impl FrogRead for UnsizedBuffer {
    fn fg_read(buf: &mut std::io::Cursor<&[u8]>) -> Result<Self, ReadError>
    where
        Self: Sized,
    {
        let left_over =
            buf.get_ref().len() - usize::try_from(buf.position()).expect("Buffer length too large");

        let mut buffer = Vec::with_capacity(left_over);
        buf.read_to_end(&mut buffer)?;

        Ok(UnsizedBuffer::from_vec(buffer))
    }
}
