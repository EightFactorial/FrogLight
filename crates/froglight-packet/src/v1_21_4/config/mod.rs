//! TODO
#![expect(missing_docs)]

#[cfg(feature = "bevy")]
use bevy_reflect::prelude::*;
use derive_more::{From, TryInto, TryUnwrap};

pub use crate::v1_21_4::play::{ClientOptionsC2SPacket, CookieResponseC2SPacket, CustomPayloadC2SPacket, KeepAliveC2SPacket, CommonPongC2SPacket, ResourcePackStatusC2SPacket, CookieRequestS2CPacket, CustomPayloadS2CPacket, DisconnectS2CPacket, KeepAliveS2CPacket, CommonPingS2CPacket, ResourcePackRemoveS2CPacket, ResourcePackSendS2CPacket, StoreCookieS2CPacket, ServerTransferS2CPacket, SynchronizeTagsS2CPacket, CustomReportDetailsS2CPacket, ServerLinksS2CPacket};

pub(super) mod c2s_0x03_finish_configuration;
pub use c2s_0x03_finish_configuration::ReadyC2SPacket;

pub(super) mod c2s_0x07_select_known_packs;
pub use c2s_0x07_select_known_packs::SelectKnownPacksC2SPacket;

pub(super) mod s2c_0x03_finish_configuration;
pub use s2c_0x03_finish_configuration::ReadyS2CPacket;

pub(super) mod s2c_0x06_reset_chat;
pub use s2c_0x06_reset_chat::ResetChatS2CPacket;

pub(super) mod s2c_0x07_registry_data;
pub use s2c_0x07_registry_data::DynamicRegistriesS2CPacket;

pub(super) mod s2c_0x0c_update_enabled_features;
pub use s2c_0x0c_update_enabled_features::FeaturesS2CPacket;

pub(super) mod s2c_0x0e_select_known_packs;
pub use s2c_0x0e_select_known_packs::SelectKnownPacksS2CPacket;


#[repr(u8)]
#[derive(Debug, Clone, PartialEq, From, TryInto, TryUnwrap)]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(Debug, Clone, PartialEq))]
#[cfg_attr(feature = "io", derive(froglight_macros::FrogPackets))]
pub enum ClientboundConfigPackets {
    ClientOptions(ClientOptionsC2SPacket) = 0x00,
    CookieResponse(CookieResponseC2SPacket) = 0x01,
    CustomPayload(CustomPayloadC2SPacket) = 0x02,
    Ready(ReadyC2SPacket) = 0x03,
    KeepAlive(KeepAliveC2SPacket) = 0x04,
    CommonPong(CommonPongC2SPacket) = 0x05,
    ResourcePackStatus(ResourcePackStatusC2SPacket) = 0x06,
    SelectKnownPacks(SelectKnownPacksC2SPacket) = 0x07,
}

#[repr(u8)]
#[derive(Debug, Clone, PartialEq, From, TryInto, TryUnwrap)]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(Debug, Clone, PartialEq))]
#[cfg_attr(feature = "io", derive(froglight_macros::FrogPackets))]
pub enum ServerboundConfigPackets {
    CookieRequest(CookieRequestS2CPacket) = 0x00,
    CustomPayload(CustomPayloadS2CPacket) = 0x01,
    Disconnect(DisconnectS2CPacket) = 0x02,
    Ready(ReadyS2CPacket) = 0x03,
    KeepAlive(KeepAliveS2CPacket) = 0x04,
    CommonPing(CommonPingS2CPacket) = 0x05,
    ResetChat(ResetChatS2CPacket) = 0x06,
    DynamicRegistries(DynamicRegistriesS2CPacket) = 0x07,
    ResourcePackRemove(ResourcePackRemoveS2CPacket) = 0x08,
    ResourcePackSend(ResourcePackSendS2CPacket) = 0x09,
    StoreCookie(StoreCookieS2CPacket) = 0x0a,
    ServerTransfer(ServerTransferS2CPacket) = 0x0b,
    Features(FeaturesS2CPacket) = 0x0c,
    SynchronizeTags(SynchronizeTagsS2CPacket) = 0x0d,
    SelectKnownPacks(SelectKnownPacksS2CPacket) = 0x0e,
    CustomReportDetails(CustomReportDetailsS2CPacket) = 0x0f,
    ServerLinks(ServerLinksS2CPacket) = 0x10,
}
