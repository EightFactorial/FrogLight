//! [`Handshaking`](crate::states::Handshaking) state packets for
//! [`V1_20_3`](super::V1_20_3)
//!
//! @generated by `froglight-generator #a28591a`
#![allow(missing_docs)]

use froglight_macros::frog_state;

pub use crate::versions::v1_20_0::handshaking::HandshakeC2SPacket;

frog_state! {
    Handshaking,
    V1_20_3,
    Serverbound {
        0u32 => HandshakeC2SPacket,
    },
}
