pub mod serverboundhandshakepacket;

use super::V1_20_1;
use crate::versions::state::Handshake;
use mc_rs_macros::impl_state;

impl_state!(
    Handshake,
    V1_20_1,
    Clientbound => {
    },
    Serverbound => {
        0x0 => serverboundhandshakepacket::ServerboundHandshakePacket,
    },
);
