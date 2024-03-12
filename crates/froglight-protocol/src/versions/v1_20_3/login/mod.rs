//! [`Login`](crate::states::Login) state packets for
//! [`V1_20_3`](super::V1_20_3)
//!
//! @generated by `froglight-generator #a28591a`
#![allow(missing_docs)]

use froglight_macros::frog_state;

mod logindisconnects2cpacket;
pub use logindisconnects2cpacket::*;

mod loginhellos2cpacket;
pub use loginhellos2cpacket::*;

mod loginsuccesss2cpacket;
pub use loginsuccesss2cpacket::*;

mod logincompressions2cpacket;
pub use logincompressions2cpacket::*;

mod loginqueryrequests2cpacket;
pub use loginqueryrequests2cpacket::*;

mod loginhelloc2spacket;
pub use loginhelloc2spacket::*;

mod loginkeyc2spacket;
pub use loginkeyc2spacket::*;

mod loginqueryresponsec2spacket;
pub use loginqueryresponsec2spacket::*;

mod enterconfigurationc2spacket;
pub use enterconfigurationc2spacket::*;

frog_state! {
	Login,
	V1_20_3,
	Clientbound {
		0u32 => LoginDisconnectS2CPacket,
		1u32 => LoginHelloS2CPacket,
		2u32 => LoginSuccessS2CPacket,
		3u32 => LoginCompressionS2CPacket,
		4u32 => LoginQueryRequestS2CPacket,
	},
	Serverbound {
		0u32 => LoginHelloC2SPacket,
		1u32 => LoginKeyC2SPacket,
		2u32 => LoginQueryResponseC2SPacket,
		3u32 => EnterConfigurationC2SPacket,
	},
}
