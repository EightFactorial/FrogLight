use std::io::Read;

use bevy_reflect::Reflect;
use derive_more::{Deref, DerefMut};
use smallvec::SmallVec;

use crate::io::{FrogRead, FrogWrite, ReadError, WriteError};

/// A buffer that contains encoded data.
///
/// Unlike a [`Vec<u8>`], when encoded it is not prefixed with a length.
///
/// When used as a field in a packet, the buffer takes up the entire length of
/// the packet.
///
/// For this reason, it *must* be the last field in the packet.
#[derive(Debug, Default, Clone, PartialEq, Eq, Deref, DerefMut, Reflect)]
pub struct UnsizedByteBuffer(SmallVec<[u8; Self::BUFFER_SIZE]>);

impl UnsizedByteBuffer {
    /// The default buffer size for an [`UnsizedByteBuffer`].
    pub const BUFFER_SIZE: usize = 16;

    /// Creates a new [`UnsizedByteBuffer`]
    #[must_use]
    pub fn new() -> Self { Self(SmallVec::new()) }

    /// Creates a new [`UnsizedByteBuffer`] with the specified capacity.
    #[must_use]
    pub fn with_capacity(capacity: usize) -> Self { Self(SmallVec::with_capacity(capacity)) }

    /// Creates a new [`UnsizedByteBuffer`] from a vector.
    #[must_use]
    pub fn from_vec(vec: Vec<u8>) -> Self { Self(SmallVec::from_vec(vec)) }

    /// Creates a new [`UnsizedByteBuffer`] from a slice.
    #[must_use]
    pub fn from_slice(slice: &[u8]) -> Self { Self(SmallVec::from_slice(slice)) }

    /// Creates a new [`UnsizedByteBuffer`] from an array.
    #[must_use]
    pub fn from_array<const N: usize>(arr: [u8; N]) -> Self {
        let mut smallvec = SmallVec::with_capacity(N);
        smallvec.extend(arr);

        Self(smallvec)
    }

    /// Creates a new [`UnsizedByteBuffer`] from a const array
    #[must_use]
    pub const fn from_const(arr: [u8; Self::BUFFER_SIZE]) -> Self {
        Self(SmallVec::from_const(arr))
    }
}

impl std::io::Write for UnsizedByteBuffer {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        <SmallVec<[u8; Self::BUFFER_SIZE]> as std::io::Write>::write(&mut self.0, buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        <SmallVec<[u8; Self::BUFFER_SIZE]> as std::io::Write>::flush(&mut self.0)
    }
}

impl std::io::Read for UnsizedByteBuffer {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let len = std::cmp::min(buf.len(), self.0.len());
        buf[..len].copy_from_slice(&self.0[..len]);
        self.0.drain(..len);
        Ok(len)
    }
}

impl AsRef<[u8]> for UnsizedByteBuffer {
    fn as_ref(&self) -> &[u8] { &self.0 }
}

impl From<Vec<u8>> for UnsizedByteBuffer {
    fn from(bytes: Vec<u8>) -> Self { Self::from_vec(bytes) }
}

impl From<&[u8]> for UnsizedByteBuffer {
    fn from(bytes: &[u8]) -> Self { Self::from_slice(bytes) }
}

impl<const N: usize> From<[u8; N]> for UnsizedByteBuffer {
    fn from(value: [u8; N]) -> Self { Self::from_array(value) }
}

impl From<UnsizedByteBuffer> for Vec<u8> {
    fn from(buffer: UnsizedByteBuffer) -> Self { buffer.0.to_vec() }
}

impl FrogWrite for UnsizedByteBuffer {
    fn fg_write(&self, buf: &mut (impl std::io::Write + ?Sized)) -> Result<(), WriteError> {
        buf.write_all(&self.0)?;
        Ok(())
    }
}

impl FrogRead for UnsizedByteBuffer {
    fn fg_read(buf: &mut std::io::Cursor<&[u8]>) -> Result<Self, ReadError>
    where
        Self: Sized,
    {
        let left_over =
            buf.get_ref().len() - usize::try_from(buf.position()).expect("Buffer length too large");

        let mut buffer = Vec::with_capacity(left_over);
        buf.read_to_end(&mut buffer)?;

        Ok(UnsizedByteBuffer::from_vec(buffer))
    }
}
