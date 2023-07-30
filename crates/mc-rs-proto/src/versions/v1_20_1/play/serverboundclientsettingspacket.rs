use mc_rs_macros::Transcode;

use crate::types::packets::client_settings::{
    ChatVisibility, ModelCustomization, SettingsMainHand,
};

#[derive(Debug, Clone, Transcode)]
pub struct ServerboundClientSettingsPacket {
    pub language: String,
    pub view_distance: u8,
    pub chat_visibility: ChatVisibility,
    pub chat_colors: bool,
    pub model: ModelCustomization,
    pub main_hand: SettingsMainHand,
    pub text_filtering: bool,
    pub allow_listing: bool,
}

impl Default for ServerboundClientSettingsPacket {
    fn default() -> Self {
        Self {
            language: "en_us".to_string(),
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
