//! @generated [`Config`](crate::version::Config) packets for v26.2

pub use crate::generated::v26_1::configuration::CookieRequestS2CPacket;

pub use crate::generated::v26_1::configuration::CustomPayloadS2CPacket;

pub use crate::generated::v26_1::configuration::DisconnectS2CPacket;

pub use crate::generated::v26_1::configuration::FinishConfigurationS2CPacket;

pub use crate::generated::v26_1::configuration::KeepAliveS2CPacket;

pub use crate::generated::v26_1::configuration::PingS2CPacket;

pub use crate::generated::v26_1::configuration::ResetChatS2CPacket;

pub use crate::generated::v26_1::configuration::RegistryDataS2CPacket;

pub use crate::generated::v26_1::configuration::ResourcePackPopS2CPacket;

pub use crate::generated::v26_1::configuration::ResourcePackPushS2CPacket;

pub use crate::generated::v26_1::configuration::StoreCookieS2CPacket;

pub use crate::generated::v26_1::configuration::TransferS2CPacket;

pub use crate::generated::v26_1::configuration::UpdateEnabledFeaturesS2CPacket;

pub use crate::generated::v26_1::configuration::UpdateTagsS2CPacket;

pub use crate::generated::v26_1::configuration::SelectKnownPacksS2CPacket;

pub use crate::generated::v26_1::configuration::CustomReportDetailsS2CPacket;

pub use crate::generated::v26_1::configuration::ServerLinksS2CPacket;

pub use crate::generated::v26_1::configuration::ClearDialogS2CPacket;

pub use crate::generated::v26_1::configuration::ShowDialogS2CPacket;

pub use crate::generated::v26_1::configuration::CodeOfConductS2CPacket;

pub use crate::generated::v26_1::configuration::ClientInformationC2SPacket;

pub use crate::generated::v26_1::configuration::CookieResponseC2SPacket;

pub use crate::generated::v26_1::configuration::CustomPayloadC2SPacket;

pub use crate::generated::v26_1::configuration::FinishConfigurationC2SPacket;

pub use crate::generated::v26_1::configuration::KeepAliveC2SPacket;

pub use crate::generated::v26_1::configuration::PongC2SPacket;

pub use crate::generated::v26_1::configuration::ResourcePackC2SPacket;

pub use crate::generated::v26_1::configuration::SelectKnownPacksC2SPacket;

pub use crate::generated::v26_1::configuration::CustomClickActionC2SPacket;

pub use crate::generated::v26_1::configuration::AcceptCodeOfConductC2SPacket;

#[repr(u8)]
#[cfg(feature = "v26_2")]
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Clone, PartialEq))]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub enum ClientboundPackets {
    CookieRequest(CookieRequestS2CPacket) = 0x00,
    CustomPayload(CustomPayloadS2CPacket) = 0x01,
    Disconnect(DisconnectS2CPacket) = 0x02,
    FinishConfiguration(FinishConfigurationS2CPacket) = 0x03,
    KeepAlive(KeepAliveS2CPacket) = 0x04,
    Ping(PingS2CPacket) = 0x05,
    ResetChat(ResetChatS2CPacket) = 0x06,
    RegistryData(RegistryDataS2CPacket) = 0x07,
    ResourcePackPop(ResourcePackPopS2CPacket) = 0x08,
    ResourcePackPush(ResourcePackPushS2CPacket) = 0x09,
    StoreCookie(StoreCookieS2CPacket) = 0x0a,
    Transfer(TransferS2CPacket) = 0x0b,
    UpdateEnabledFeatures(UpdateEnabledFeaturesS2CPacket) = 0x0c,
    UpdateTags(UpdateTagsS2CPacket) = 0x0d,
    SelectKnownPacks(SelectKnownPacksS2CPacket) = 0x0e,
    CustomReportDetails(CustomReportDetailsS2CPacket) = 0x0f,
    ServerLinks(ServerLinksS2CPacket) = 0x10,
    ClearDialog(ClearDialogS2CPacket) = 0x11,
    ShowDialog(ShowDialogS2CPacket) = 0x12,
    CodeOfConduct(CodeOfConductS2CPacket) = 0x13,
}

#[repr(u8)]
#[cfg(feature = "v26_2")]
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Clone, PartialEq))]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub enum ServerboundPackets {
    ClientInformation(ClientInformationC2SPacket) = 0x00,
    CookieResponse(CookieResponseC2SPacket) = 0x01,
    CustomPayload(CustomPayloadC2SPacket) = 0x02,
    FinishConfiguration(FinishConfigurationC2SPacket) = 0x03,
    KeepAlive(KeepAliveC2SPacket) = 0x04,
    Pong(PongC2SPacket) = 0x05,
    ResourcePack(ResourcePackC2SPacket) = 0x06,
    SelectKnownPacks(SelectKnownPacksC2SPacket) = 0x07,
    CustomClickAction(CustomClickActionC2SPacket) = 0x08,
    AcceptCodeOfConduct(AcceptCodeOfConductC2SPacket) = 0x09,
}
