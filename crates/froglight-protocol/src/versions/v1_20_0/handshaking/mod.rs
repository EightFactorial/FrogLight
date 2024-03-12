//! [`Handshaking`](crate::states::Handshaking) state packets for
//! [`V1_20_0`](super::V1_20_0)
//!
//! @generated by `froglight-generator #a28591a`
#![allow(missing_docs)]

use froglight_macros::frog_state;

mod handshakec2spacket;
pub use handshakec2spacket::*;

frog_state! {
	Handshaking,
	V1_20_0,
	Serverbound {
		0u32 => HandshakeC2SPacket,
	},
}
