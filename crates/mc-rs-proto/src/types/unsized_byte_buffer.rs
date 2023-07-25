use bevy_derive::{Deref, DerefMut};

use crate::buffer::{Decode, DecodeError, Encode, EncodeError};

/// A buffer that contains encoded data.
///
/// The buffer takes up the entire remaining space of the packet.
///
/// For this reason, it is not possible to use this type in a packet
/// that contains other fields after it.
///
/// For example:
/// ```rust
/// # use mc_rs_proto::types::UnsizedByteBuffer;
///
/// #[derive(Debug)]
/// struct Packet {
///     field: u8,
///     buffer: UnsizedByteBuffer,
/// }
/// ```
///
/// If the packet length is 8 bytes, the buffer contains 7 bytes of data.
#[derive(Debug, Default, Clone, Deref, DerefMut)]
pub struct UnsizedByteBuffer(Vec<u8>);

impl UnsizedByteBuffer {
    pub fn new() -> Self { Self(Vec::new()) }

    pub fn with_capacity(capacity: usize) -> Self { Self(Vec::with_capacity(capacity)) }

    pub fn write_value<T: Encode>(&mut self, value: T) -> Result<(), EncodeError> {
        value.encode(&mut self.0)
    }
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
        Ok(Self(bytes))
    }
}
