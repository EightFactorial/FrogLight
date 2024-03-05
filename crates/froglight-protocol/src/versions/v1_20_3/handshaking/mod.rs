//! [`Handshaking`] state packets for [`1.20.3`]
//!
//! @generated by `froglight-generator #25841ed`
#![allow(missing_docs)]

use froglight_macros::frog_state;

mod handshakec2spacket;
pub use handshakec2spacket::*;

frog_state! {
	Handshaking,
	V1_20_3,
	Serverbound {
		0u32 => HandshakeC2SPacket,
	},
}
