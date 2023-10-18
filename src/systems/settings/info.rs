use bevy::prelude::*;
use compact_str::CompactString;
use mc_rs_protocol::{
    types::packets::client_settings::{ChatVisibility, ModelCustomization, SettingsMainHand},
    versions::v1_20_0::play::serverboundclientsettingspacket::ServerboundClientSettingsPacket as V1_20_0_ServerboundClientSettingsPacket,
};

#[derive(Debug, Clone, Resource)]
pub struct ClientInformation {
    pub language: CompactString,
    pub view_distance: u8,
    pub chat_visibility: ChatVisibility,
    pub chat_colors: bool,
    pub model: ModelCustomization,
    pub main_hand: SettingsMainHand,
    pub text_filtering: bool,
    pub allow_listing: bool,
}

impl Default for ClientInformation {
    fn default() -> Self {
        Self {
            language: CompactString::new_inline("en_US"),
            view_distance: 8,
            chat_visibility: ChatVisibility::default(),
            chat_colors: true,
            model: ModelCustomization::default(),
            main_hand: SettingsMainHand::Right,
            text_filtering: false,
            allow_listing: false,
        }
    }
}

impl From<ClientInformation> for V1_20_0_ServerboundClientSettingsPacket {
    fn from(value: ClientInformation) -> Self {
        Self {
            language: value.language,
            view_distance: value.view_distance,
            chat_visibility: value.chat_visibility,
            chat_colors: value.chat_colors,
            model: value.model,
            main_hand: value.main_hand,
            text_filtering: value.text_filtering,
            allow_listing: value.allow_listing,
        }
    }
}
