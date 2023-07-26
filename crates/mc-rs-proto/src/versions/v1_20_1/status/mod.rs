use derive_more::From;

use crate::{
    buffer::{Decode, Encode, VarDecode, VarEncode},
    versions::state::Status,
    State,
};

use super::V1_20_1;

pub mod clientboundquerypongpacket;
pub mod clientboundqueryresponsepacket;
pub mod serverboundquerypingpacket;
pub mod serverboundqueryrequestpacket;

// TODO: Write state macro

impl State<V1_20_1> for Status {
    type Clientbound = ClientboundStatusPackets;
    type Serverbound = ServerboundStatusPackets;
}

#[derive(Debug, Clone, From)]
pub enum ClientboundStatusPackets {
    StatusResponse(clientboundqueryresponsepacket::ClientboundQueryResponsePacket),
    PongResponse(clientboundquerypongpacket::ClientboundQueryPongPacket),
}

impl Encode for ClientboundStatusPackets {
    fn encode(&self, buf: &mut impl std::io::Write) -> Result<(), crate::buffer::EncodeError> {
        match self {
            ClientboundStatusPackets::StatusResponse(packet) => {
                0u32.var_encode(buf)?;
                packet.encode(buf)
            }
            ClientboundStatusPackets::PongResponse(packet) => {
                1u32.var_encode(buf)?;
                packet.encode(buf)
            }
        }
    }
}

impl Decode for ClientboundStatusPackets {
    fn decode(buf: &mut impl std::io::Read) -> Result<Self, crate::buffer::DecodeError> {
        let id = u32::var_decode(buf)?;
        match id {
            0 => Ok(ClientboundStatusPackets::StatusResponse(
                clientboundqueryresponsepacket::ClientboundQueryResponsePacket::decode(buf)?,
            )),
            1 => Ok(ClientboundStatusPackets::PongResponse(
                clientboundquerypongpacket::ClientboundQueryPongPacket::decode(buf)?,
            )),
            _ => Err(crate::buffer::DecodeError::UnknownPacketId(id)),
        }
    }
}

#[derive(Debug, Clone, From)]
pub enum ServerboundStatusPackets {
    StatusRequest(serverboundqueryrequestpacket::ServerboundQueryRequestPacket),
    PingRequest(serverboundquerypingpacket::ServerboundQueryPingPacket),
}

impl Encode for ServerboundStatusPackets {
    fn encode(&self, buf: &mut impl std::io::Write) -> Result<(), crate::buffer::EncodeError> {
        match self {
            ServerboundStatusPackets::StatusRequest(packet) => {
                0u32.var_encode(buf)?;
                packet.encode(buf)
            }
            ServerboundStatusPackets::PingRequest(packet) => {
                1u32.var_encode(buf)?;
                packet.encode(buf)
            }
        }
    }
}

impl Decode for ServerboundStatusPackets {
    fn decode(buf: &mut impl std::io::Read) -> Result<Self, crate::buffer::DecodeError> {
        let id = u32::var_decode(buf)?;
        match id {
            0 => Ok(ServerboundStatusPackets::StatusRequest(
                serverboundqueryrequestpacket::ServerboundQueryRequestPacket::decode(buf)?,
            )),
            1 => Ok(ServerboundStatusPackets::PingRequest(
                serverboundquerypingpacket::ServerboundQueryPingPacket::decode(buf)?,
            )),
            _ => Err(crate::buffer::DecodeError::UnknownPacketId(id)),
        }
    }
}
