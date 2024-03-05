//! [`Status`] state packets for [`1.20.0`]
//!
//! @generated by `froglight-generator #0ff4b67`
#![allow(missing_docs)]

use froglight_macros::frog_state;

mod queryresponses2cpacket;
pub use queryresponses2cpacket::*;

mod querypongs2cpacket;
pub use querypongs2cpacket::*;

mod queryrequestc2spacket;
pub use queryrequestc2spacket::*;

mod querypingc2spacket;
pub use querypingc2spacket::*;

frog_state! {
	Status,
	V1_20_0,
	Clientbound {
		0u32 => QueryResponseS2CPacket,
		1u32 => QueryPongS2CPacket,
	},
	Serverbound {
		0u32 => QueryRequestC2SPacket,
		1u32 => QueryPingC2SPacket,
	},
}
