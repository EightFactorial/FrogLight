use derive_more::{Deref, DerefMut};
use mc_rs_macros::Test;
use smallvec::SmallVec;

use crate::buffer::{Decode, DecodeError, Encode, EncodeError, FromValue, VarEncode};

/// A buffer that contains encoded data.
///
/// Unlike a [`Vec<u8>`], when encoded it is not prefixed with a length.
///
/// When used as a field in a packet, the buffer takes up the entire remaining space of the packet.
///
/// For this reason, it must be the last field in the packet.
///
/// For example:
/// ```rust
/// use mc_rs_macros::Transcode;
/// use mc_rs_protocol::{types::UnsizedByteBuffer, buffer::{Encode, Decode, FromValue}};
/// use compact_str::CompactString;
///
/// let string = CompactString::new("Hello, world!");
///
/// let unsized_buffer = UnsizedByteBuffer::from_value(&string).unwrap();
/// let encode_buffer = Vec::from_value(&unsized_buffer);
///
/// // Note that the buffer is not prefixed with a length, though the contained string data is.
/// assert_eq!(encode_buffer.unwrap(), [13, 72, 101, 108, 108, 111, 44, 32, 119, 111, 114, 108, 100, 33]);
///
/// // The buffer contains 16u32, which is 4 bytes.
/// let mut unsized_buffer = UnsizedByteBuffer::from([0, 0, 0, 16]);
/// let decoded = u32::decode(&mut unsized_buffer).unwrap();
///
/// // The buffer now contains 0 bytes.
/// assert_eq!(decoded, 16);
/// assert!(unsized_buffer.is_empty());
/// ```
#[derive(Debug, Default, Clone, PartialEq, Eq, Deref, DerefMut, Test)]
#[mctest(tests = ["transcode"])]
pub struct UnsizedByteBuffer(SmallVec<[u8; Self::BUFFER_SIZE]>);

impl UnsizedByteBuffer {
    /// The default buffer size for an [UnsizedByteBuffer].
    pub const BUFFER_SIZE: usize = 16;

    /// Creates a new [UnsizedByteBuffer]
    pub fn new() -> Self { Self(SmallVec::new()) }

    /// Creates a new [UnsizedByteBuffer] with the specified capacity.
    pub fn with_capacity(capacity: usize) -> Self { Self(SmallVec::with_capacity(capacity)) }

    /// Creates a new [UnsizedByteBuffer] from a vector.
    pub fn from_vec(vec: Vec<u8>) -> Self { Self(SmallVec::from_vec(vec)) }

    /// Creates a new [UnsizedByteBuffer] from a slice.
    pub fn from_slice(slice: &[u8]) -> Self { Self(SmallVec::from_slice(slice)) }

    /// Creates a new [UnsizedByteBuffer] from an array.
    pub fn from_array<const N: usize>(arr: [u8; N]) -> Self {
        let mut smallvec = SmallVec::with_capacity(N);
        smallvec.extend(arr);

        Self(smallvec)
    }

    /// Creates a new [UnsizedByteBuffer] from a const array
    pub const fn from_const(arr: [u8; Self::BUFFER_SIZE]) -> Self {
        Self(SmallVec::from_const(arr))
    }
}

impl FromValue for UnsizedByteBuffer {
    fn from_value<T: Encode>(value: &T) -> Result<Self, EncodeError> {
        let mut buffer = Self::new();
        value.encode(&mut buffer)?;
        Ok(buffer)
    }

    fn from_var_value<T: VarEncode>(value: &T) -> Result<Self, EncodeError> {
        let mut buffer = Self::new();
        value.var_encode(&mut buffer)?;
        Ok(buffer)
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

impl Encode for UnsizedByteBuffer {
    fn encode(&self, buf: &mut impl std::io::Write) -> Result<(), EncodeError> {
        buf.write_all(&self.0)?;
        Ok(())
    }
}

impl Decode for UnsizedByteBuffer {
    fn decode(buf: &mut impl std::io::Read) -> Result<Self, DecodeError> {
        let mut bytes = Vec::new();
        buf.read_to_end(&mut bytes)?;

        Ok(Self::from_vec(bytes))
    }
}
