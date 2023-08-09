pub mod clientboundlogincompressionpacket;
pub mod clientboundlogindisconnectpacket;
pub mod clientboundloginhellopacket;
pub mod clientboundloginqueryrequestpacket;
pub mod clientboundloginsuccesspacket;
pub mod serverboundloginhellopacket;
pub mod serverboundloginkeypacket;
pub mod serverboundloginqueryresponsepacket;

use super::V1_20_0;
use crate::versions::state::Login;
use mc_rs_macros::impl_state;

impl_state!(
    Login,
    V1_20_0,
    Clientbound => {
        0x0 => clientboundlogindisconnectpacket::ClientboundLoginDisconnectPacket,
        0x1 => clientboundloginhellopacket::ClientboundLoginHelloPacket,
        0x2 => clientboundloginsuccesspacket::ClientboundLoginSuccessPacket,
        0x3 => clientboundlogincompressionpacket::ClientboundLoginCompressionPacket,
        0x4 => clientboundloginqueryrequestpacket::ClientboundLoginQueryRequestPacket,
    },
    Serverbound => {
        0x0 => serverboundloginhellopacket::ServerboundLoginHelloPacket,
        0x1 => serverboundloginkeypacket::ServerboundLoginKeyPacket,
        0x2 => serverboundloginqueryresponsepacket::ServerboundLoginQueryResponsePacket,
    },
);
