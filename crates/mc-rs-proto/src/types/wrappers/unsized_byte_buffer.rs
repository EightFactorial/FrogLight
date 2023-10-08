use derive_more::{Deref, DerefMut};
use smallvec::SmallVec;

use crate::buffer::{Decode, DecodeError, Encode, EncodeError, VarEncode};

/// A buffer that contains encoded data.
///
/// The buffer takes up the entire remaining space of the packet.
///
/// Unlike a `Vec<T>`, it is not prefixed with a length.
/// For this reason, it is not possible to use this type in a packet
/// that contains other fields after it.
///
/// For example:
/// ```rust,ignore
/// use mc_rs_macros::Transcode;
/// use mc_rs_proto::types::UnsizedByteBuffer;
///
/// #[derive(Debug, Transcode)]
/// struct Packet {
///     field: u8,
///     buffer: UnsizedByteBuffer,
/// }
/// ```
///
/// If the packet length is 8 bytes, the buffer contains 7 bytes of data.
#[derive(Debug, Default, Clone, Deref, DerefMut)]
pub struct UnsizedByteBuffer(SmallVec<[u8; 16]>);

impl UnsizedByteBuffer {
    pub fn new() -> Self { Self(SmallVec::new()) }

    pub fn with_capacity(capacity: usize) -> Self { Self(SmallVec::with_capacity(capacity)) }

    pub fn encode_value<T: Encode>(&mut self, value: &T) -> Result<(), EncodeError> {
        value.encode(&mut self.0)
    }

    pub fn var_encode_value<T: VarEncode>(&mut self, value: &T) -> Result<(), EncodeError> {
        value.var_encode(&mut self.0)
    }

    pub fn from_value<T: Encode>(value: &T) -> Result<Self, EncodeError> {
        let mut buffer = Self::new();
        buffer.encode_value(value)?;
        Ok(buffer)
    }

    pub fn from_var_value<T: VarEncode>(value: &T) -> Result<Self, EncodeError> {
        let mut buffer = Self::new();
        buffer.var_encode_value(value)?;
        Ok(buffer)
    }
}

impl From<Vec<u8>> for UnsizedByteBuffer {
    fn from(bytes: Vec<u8>) -> Self { Self(SmallVec::from_vec(bytes)) }
}

impl From<UnsizedByteBuffer> for Vec<u8> {
    fn from(buffer: UnsizedByteBuffer) -> Self { buffer.0.to_vec() }
}

impl From<&[u8]> for UnsizedByteBuffer {
    fn from(bytes: &[u8]) -> Self { Self(SmallVec::from_slice(bytes)) }
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

        Ok(Self(SmallVec::from_vec(bytes)))
    }
}
