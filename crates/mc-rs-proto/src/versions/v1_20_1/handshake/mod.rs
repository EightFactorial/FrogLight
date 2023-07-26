use derive_more::From;

use crate::{
    buffer::{Decode, Encode, VarDecode, VarEncode},
    versions::state::Handshake,
    State,
};

use super::V1_20_1;

pub mod serverboundhandshakepacket;

// TODO: Write state macro

impl State<V1_20_1> for Handshake {
    type Clientbound = ClientboundHandshakePackets;
    type Serverbound = ServerboundHandshakePackets;
}

#[derive(Debug, Clone)]
pub enum ClientboundHandshakePackets {}

impl Decode for ClientboundHandshakePackets {
    fn decode(_buf: &mut impl std::io::Read) -> Result<Self, crate::buffer::DecodeError> {
        panic!("There are no clientbound packets in the handshake state")
    }
}

#[derive(Debug, Clone, From)]
pub enum ServerboundHandshakePackets {
    Handshake(serverboundhandshakepacket::ServerboundHandshakePacket),
}

impl Encode for ServerboundHandshakePackets {
    fn encode(&self, buf: &mut impl std::io::Write) -> Result<(), crate::buffer::EncodeError> {
        match self {
            ServerboundHandshakePackets::Handshake(packet) => {
                0u32.var_encode(buf)?;
                packet.encode(buf)
            }
        }
    }
}

impl Decode for ServerboundHandshakePackets {
    fn decode(buf: &mut impl std::io::Read) -> Result<Self, crate::buffer::DecodeError> {
        let id = u32::var_decode(buf)?;
        match id {
            0 => Ok(ServerboundHandshakePackets::Handshake(
                serverboundhandshakepacket::ServerboundHandshakePacket::decode(buf)?,
            )),
            _ => Err(crate::buffer::DecodeError::UnknownPacketId(id)),
        }
    }
}
