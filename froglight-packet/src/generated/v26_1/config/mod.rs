//! @generated [`Config`](crate::version::Config) packets for v26.1.x

mod c2s_0x00_client_information;
pub use c2s_0x00_client_information::ClientInformationC2SPacket;

mod c2s_0x01_cookie_response;
pub use c2s_0x01_cookie_response::CookieResponseC2SPacket;

mod c2s_0x02_custom_payload;
pub use c2s_0x02_custom_payload::QueryResponseC2SPacket;

mod c2s_0x03_finish_configuration;
pub use c2s_0x03_finish_configuration::FinishConfigurationC2SPacket;

mod c2s_0x04_keep_alive;
pub use c2s_0x04_keep_alive::KeepAliveC2SPacket;

mod c2s_0x05_pong;
pub use c2s_0x05_pong::PongC2SPacket;

mod c2s_0x06_resource_pack;
pub use c2s_0x06_resource_pack::ResourcePackC2SPacket;

mod c2s_0x07_select_known_packs;
pub use c2s_0x07_select_known_packs::SelectKnownPacksC2SPacket;

mod c2s_0x08_custom_click_action;
pub use c2s_0x08_custom_click_action::CustomClickActionC2SPacket;

mod c2s_0x09_accept_code_of_conduct;
pub use c2s_0x09_accept_code_of_conduct::AcceptCodeOfConductC2SPacket;

mod s2c_0x00_cookie_request;
pub use s2c_0x00_cookie_request::CookieQueryS2CPacket;

mod s2c_0x01_custom_payload;
pub use s2c_0x01_custom_payload::CustomQueryS2CPacket;

mod s2c_0x02_disconnect;
pub use s2c_0x02_disconnect::ConfigDisconnectS2CPacket;

mod s2c_0x03_finish_configuration;
pub use s2c_0x03_finish_configuration::FinishConfigurationS2CPacket;

mod s2c_0x04_keep_alive;
pub use s2c_0x04_keep_alive::KeepAliveS2CPacket;

mod s2c_0x05_ping;
pub use s2c_0x05_ping::PingS2CPacket;

mod s2c_0x06_reset_chat;
pub use s2c_0x06_reset_chat::ResetChatS2CPacket;

mod s2c_0x07_registry_data;
pub use s2c_0x07_registry_data::RegistryDataS2CPacket;

mod s2c_0x08_resource_pack_pop;
pub use s2c_0x08_resource_pack_pop::ResourcePackPopS2CPacket;

mod s2c_0x09_resource_pack_push;
pub use s2c_0x09_resource_pack_push::ResourcePackPushS2CPacket;

mod s2c_0x0a_store_cookie;
pub use s2c_0x0a_store_cookie::StoreCookieS2CPacket;

mod s2c_0x0b_transfer;
pub use s2c_0x0b_transfer::TransferS2CPacket;

mod s2c_0x0c_update_enabled_features;
pub use s2c_0x0c_update_enabled_features::UpdateEnabledFeaturesS2CPacket;

mod s2c_0x0d_update_tags;
pub use s2c_0x0d_update_tags::UpdateTagsS2CPacket;

mod s2c_0x0e_select_known_packs;
pub use s2c_0x0e_select_known_packs::SelectKnownPacksS2CPacket;

mod s2c_0x0f_custom_report_details;
pub use s2c_0x0f_custom_report_details::CustomReportDetailsS2CPacket;

mod s2c_0x10_server_links;
pub use s2c_0x10_server_links::ServerLinksS2CPacket;

mod s2c_0x11_clear_dialog;
pub use s2c_0x11_clear_dialog::ClearDialogS2CPacket;

mod s2c_0x12_show_dialog;
pub use s2c_0x12_show_dialog::ShowDialogS2CPacket;

mod s2c_0x13_code_of_conduct;
pub use s2c_0x13_code_of_conduct::CodeOfConductS2CPacket;

#[repr(u8)]
#[cfg(feature = "v26_1")]
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Clone, PartialEq))]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub enum ClientboundPackets {
    CookieRequest(CookieQueryS2CPacket) = 0x00,
    QueryRequest(CustomQueryS2CPacket) = 0x01,
    Disconnect(ConfigDisconnectS2CPacket) = 0x02,
    FinishConfiguration(FinishConfigurationS2CPacket) = 0x03,
    KeepAlive(KeepAliveS2CPacket) = 0x04,
    Ping(PingS2CPacket) = 0x05,
    ResetChat(ResetChatS2CPacket) = 0x06,
    RegistryData(RegistryDataS2CPacket) = 0x07,
    ResourcePackPop(ResourcePackPopS2CPacket) = 0x08,
    ResourcePackPush(ResourcePackPushS2CPacket) = 0x09,
    StoreCookie(StoreCookieS2CPacket) = 0x0A,
    Transfer(TransferS2CPacket) = 0x0B,
    UpdateEnabledFeatures(UpdateEnabledFeaturesS2CPacket) = 0x0C,
    UpdateTags(UpdateTagsS2CPacket) = 0x0D,
    SelectKnownPacks(SelectKnownPacksS2CPacket) = 0x0E,
    CustomReportDetails(CustomReportDetailsS2CPacket) = 0x0F,
    ServerLinks(ServerLinksS2CPacket) = 0x10,
    ClearDialog(ClearDialogS2CPacket) = 0x11,
    ShowDialog(ShowDialogS2CPacket) = 0x12,
    CodeOfConduct(CodeOfConductS2CPacket) = 0x13,
}

#[repr(u8)]
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Clone, PartialEq))]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub enum ServerboundPackets {
    ClientInformation(ClientInformationC2SPacket) = 0x00,
    CookieResponse(CookieResponseC2SPacket) = 0x01,
    QueryResponse(QueryResponseC2SPacket) = 0x02,
    FinishConfiguration(FinishConfigurationC2SPacket) = 0x03,
    KeepAlive(KeepAliveC2SPacket) = 0x04,
    Pong(PongC2SPacket) = 0x05,
    ResourcePack(ResourcePackC2SPacket) = 0x06,
    SelectKnownPacks(SelectKnownPacksC2SPacket) = 0x07,
    CustomClickAction(CustomClickActionC2SPacket) = 0x08,
    AcceptCodeOfConduct(AcceptCodeOfConductC2SPacket) = 0x09,
}
