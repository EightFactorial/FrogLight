use crate::{
    buffer::{Decode, DecodeError, Encode, EncodeError},
    versions::state::Handshake,
    State,
};

use super::V1_20_1;

impl State<V1_20_1> for Handshake {
    type Serverbound = HandshakeServerbound;
    type Clientbound = HandshakeClientbound;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HandshakeServerbound {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HandshakeClientbound {}

impl Encode for HandshakeServerbound {
    fn encode(&self, _buf: &mut impl std::io::Write) -> Result<(), EncodeError> { todo!() }
}

impl Decode for HandshakeClientbound {
    fn decode(_buf: &mut impl std::io::Read) -> Result<Self, DecodeError> { todo!() }
}
