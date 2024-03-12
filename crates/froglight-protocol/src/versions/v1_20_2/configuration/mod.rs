//! [`Configuration`](crate::states::Configuration) state packets for
//! [`V1_20_2`](super::V1_20_2)
//!
//! @generated by `froglight-generator #a28591a`
#![allow(missing_docs)]

use froglight_macros::frog_state;

mod custompayloads2cpacket;
pub use custompayloads2cpacket::*;

mod disconnects2cpacket;
pub use disconnects2cpacket::*;

mod readys2cpacket;
pub use readys2cpacket::*;

mod keepalives2cpacket;
pub use keepalives2cpacket::*;

mod commonpings2cpacket;
pub use commonpings2cpacket::*;

mod dynamicregistriess2cpacket;
pub use dynamicregistriess2cpacket::*;

mod resourcepacksends2cpacket;
pub use resourcepacksends2cpacket::*;

mod featuress2cpacket;
pub use featuress2cpacket::*;

mod synchronizetagss2cpacket;
pub use synchronizetagss2cpacket::*;

mod clientoptionsc2spacket;
pub use clientoptionsc2spacket::*;

mod custompayloadc2spacket;
pub use custompayloadc2spacket::*;

mod readyc2spacket;
pub use readyc2spacket::*;

mod keepalivec2spacket;
pub use keepalivec2spacket::*;

mod commonpongc2spacket;
pub use commonpongc2spacket::*;

mod resourcepackstatusc2spacket;
pub use resourcepackstatusc2spacket::*;

frog_state! {
	Configuration,
	V1_20_2,
	Clientbound {
		0u32 => CustomPayloadS2CPacket,
		1u32 => DisconnectS2CPacket,
		2u32 => ReadyS2CPacket,
		3u32 => KeepAliveS2CPacket,
		4u32 => CommonPingS2CPacket,
		5u32 => DynamicRegistriesS2CPacket,
		6u32 => ResourcePackSendS2CPacket,
		7u32 => FeaturesS2CPacket,
		8u32 => SynchronizeTagsS2CPacket,
	},
	Serverbound {
		0u32 => ClientOptionsC2SPacket,
		1u32 => CustomPayloadC2SPacket,
		2u32 => ReadyC2SPacket,
		3u32 => KeepAliveC2SPacket,
		4u32 => CommonPongC2SPacket,
		5u32 => ResourcePackStatusC2SPacket,
	},
}
