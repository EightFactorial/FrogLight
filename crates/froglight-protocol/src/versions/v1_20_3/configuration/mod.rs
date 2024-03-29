//! [`Configuration`](crate::states::Configuration) state packets for
//! [`V1_20_3`](super::V1_20_3)
//!
//! @generated by `froglight-generator #a28591a`
#![allow(missing_docs)]

use froglight_macros::frog_state;

pub use super::{
    login::LoginDisconnectS2CPacket as DisconnectS2CPacket,
    play::{
        ClientOptionsC2SPacket, KeepAliveC2SPacket, KeepAliveS2CPacket,
        ResourcePackRemoveS2CPacket, ResourcePackSendS2CPacket, ResourcePackStatusC2SPacket,
    },
};
pub use crate::versions::v1_20_2::configuration::{
    CommonPingS2CPacket, CommonPongC2SPacket, CustomPayloadC2SPacket, CustomPayloadS2CPacket,
    DynamicRegistriesS2CPacket, FeaturesS2CPacket, ReadyC2SPacket, ReadyS2CPacket,
    SynchronizeTagsS2CPacket,
};

frog_state! {
    Configuration,
    V1_20_3,
    Clientbound {
        0u32 => CustomPayloadS2CPacket,
        1u32 => DisconnectS2CPacket,
        2u32 => ReadyS2CPacket,
        3u32 => KeepAliveS2CPacket,
        4u32 => CommonPingS2CPacket,
        5u32 => DynamicRegistriesS2CPacket,
        6u32 => ResourcePackRemoveS2CPacket,
        7u32 => ResourcePackSendS2CPacket,
        8u32 => FeaturesS2CPacket,
        9u32 => SynchronizeTagsS2CPacket,
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
