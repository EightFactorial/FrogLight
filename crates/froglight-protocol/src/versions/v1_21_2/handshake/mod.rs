//! [`Handshake`](crate::states::Handshake) state packets for
//! [`V1_21_2`](super::V1_21_2)
#![allow(missing_docs)]

pub use crate::versions::v1_21_0::handshake::HandshakePacket;

froglight_macros::frog_state! {
    Handshake,
    V1_21_2,
    Clientbound {},
    Serverbound {
        0u32 => HandshakePacket,
    },
}
