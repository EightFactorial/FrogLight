//! [`Handshake`](crate::states::Handshake) state packets for
//! [`V1_21_0`](super::V1_21_0)
#![allow(missing_docs)]

mod handshake;
pub use handshake::*;

froglight_macros::frog_state! {
    Handshake,
    V1_21_0,
    Clientbound {},
    Serverbound {
        0u32 => HandshakePacket,
    },
}
