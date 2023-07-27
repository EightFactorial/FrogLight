pub mod clientboundquerypongpacket;
pub mod clientboundqueryresponsepacket;
pub mod serverboundquerypingpacket;
pub mod serverboundqueryrequestpacket;

use super::V1_20_1;
use crate::versions::state::Status;
use mc_rs_macros::impl_state;

impl_state!(
    Status,
    V1_20_1,
    Clientbound => {
        0x0 => clientboundqueryresponsepacket::ClientboundQueryResponsePacket,
        0x1 => clientboundquerypongpacket::ClientboundQueryPongPacket,
    },
    Serverbound => {
        0x0 => serverboundqueryrequestpacket::ServerboundQueryRequestPacket,
        0x1 => serverboundquerypingpacket::ServerboundQueryPingPacket,
    },
);
