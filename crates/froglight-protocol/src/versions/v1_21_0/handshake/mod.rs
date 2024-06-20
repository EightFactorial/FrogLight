//! [`Handshaking`](crate::states::Handshaking) state packets for
//! [`V1_21_0`](super::V1_21_0)
//!
//! @generated by `froglight-generator` #cff0bd9
#![allow(missing_docs)]

mod handshake;
pub use handshake::*;

froglight_macros::frog_state! {
    Handshaking,
    V1_21_0,
    Clientbound {},
    Serverbound {
        0u32 => HandshakePacket,
    },
}
