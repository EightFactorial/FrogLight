//! [`Configuration`](crate::states::Configuration) state packets for
//! [`V1_21_0`](super::V1_21_0)
#![allow(missing_docs)]

pub use super::play::{
    ClientOptionsPacket, CommonPingPacket, CommonPongPacket, CookieRequestPacket,
    CookieResponsePacket, CustomPayloadC2SPacket, CustomPayloadS2CPacket,
    CustomReportDetailsPacket, DisconnectPacket, KeepAliveC2SPacket, KeepAliveS2CPacket,
    ResourcePackRemovePacket, ResourcePackSendPacket, ResourcePackStatusPacket, ServerLinksPacket,
    ServerTransferPacket, StoreCookiePacket, SynchronizeTagsPacket,
};

mod dynamic_registries;
pub use dynamic_registries::*;

mod features;
pub use features::*;

mod ready_c2s;
pub use ready_c2s::*;

mod ready_s2c;
pub use ready_s2c::*;

mod reset_chat;
pub use reset_chat::*;

mod select_known_packs_c2s;
pub use select_known_packs_c2s::*;

mod select_known_packs_s2c;
pub use select_known_packs_s2c::*;

froglight_macros::frog_state! {
    Configuration,
    V1_21_0,
    Clientbound {
        0u32 => CookieRequestPacket,
        1u32 => CustomPayloadS2CPacket,
        2u32 => DisconnectPacket,
        3u32 => ReadyS2CPacket,
        4u32 => KeepAliveS2CPacket,
        5u32 => CommonPingPacket,
        6u32 => ResetChatPacket,
        7u32 => DynamicRegistriesPacket,
        8u32 => ResourcePackRemovePacket,
        9u32 => ResourcePackSendPacket,
        10u32 => StoreCookiePacket,
        11u32 => ServerTransferPacket,
        12u32 => FeaturesPacket,
        13u32 => SynchronizeTagsPacket,
        14u32 => SelectKnownPacksS2CPacket,
        15u32 => CustomReportDetailsPacket,
        16u32 => ServerLinksPacket,
    },
    Serverbound {
        0u32 => ClientOptionsPacket,
        1u32 => CookieResponsePacket,
        2u32 => CustomPayloadC2SPacket,
        3u32 => ReadyC2SPacket,
        4u32 => KeepAliveC2SPacket,
        5u32 => CommonPongPacket,
        6u32 => ResourcePackStatusPacket,
        7u32 => SelectKnownPacksC2SPacket,
    },
}
