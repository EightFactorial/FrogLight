//! [`Configuration`](crate::states::Configuration) state packets for
//! [`V1_21_0`](super::V1_21_0)
//!
//! @generated by `froglight-generator` #ecfea09
#![allow(missing_docs)]

mod cookie_request;
pub use cookie_request::*;

mod custom_payload;
pub use custom_payload::*;

mod custom_report_details;
pub use custom_report_details::*;

mod disconnect;
pub use disconnect::*;

mod ready;
pub use ready::*;

mod keep_alive;
pub use keep_alive::*;

mod common_ping;
pub use common_ping::*;

mod dynamic_registries;
pub use dynamic_registries::*;

mod reset_chat;
pub use reset_chat::*;

mod resource_pack_remove;
pub use resource_pack_remove::*;

mod resource_pack_send;
pub use resource_pack_send::*;

mod select_known_packs;
pub use select_known_packs::*;

mod server_links;
pub use server_links::*;

mod store_cookie;
pub use store_cookie::*;

mod server_transfer;
pub use server_transfer::*;

mod features;
pub use features::*;

mod synchronize_tags;
pub use synchronize_tags::*;

mod client_options;
pub use client_options::*;

mod cookie_response;
pub use cookie_response::*;

mod common_pong;
pub use common_pong::*;

mod resource_pack_status;
pub use resource_pack_status::*;

froglight_macros::frog_state! {
    Configuration,
    V1_21_0,
    Clientbound {
        0u32 => CookieRequestPacket,
        1u32 => CustomPayloadPacket,
        2u32 => CustomReportDetailsPacket,
        3u32 => DisconnectPacket,
        4u32 => ReadyPacket,
        5u32 => KeepAlivePacket,
        6u32 => CommonPingPacket,
        7u32 => DynamicRegistriesPacket,
        8u32 => ResetChatPacket,
        9u32 => ResourcePackRemovePacket,
        10u32 => ResourcePackSendPacket,
        11u32 => SelectKnownPacksPacket,
        12u32 => ServerLinksPacket,
        13u32 => StoreCookiePacket,
        14u32 => ServerTransferPacket,
        15u32 => FeaturesPacket,
        16u32 => SynchronizeTagsPacket,
    },
    Serverbound {
        0u32 => ClientOptionsPacket,
        1u32 => CookieResponsePacket,
        2u32 => CustomPayloadPacket,
        3u32 => ReadyPacket,
        4u32 => KeepAlivePacket,
        5u32 => CommonPongPacket,
        6u32 => ResourcePackStatusPacket,
        7u32 => SelectKnownPacksPacket,
    },
}
