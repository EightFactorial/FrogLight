use compact_str::CompactString;
use mc_rs_macros::Transcode;

use crate::types::packets::client_settings::{
    ChatVisibility, ModelCustomization, SettingsMainHand,
};

#[derive(Debug, Clone, PartialEq, Eq, Transcode)]
#[mctest(tests = ["transcode", "decode"], bytes = [5, 101, 110, 95, 85, 83, 8, 0, 1, 127, 1, 0, 1])]
pub struct ServerboundClientSettingsPacket {
    pub language: CompactString,
    pub view_distance: u8,
    pub chat_visibility: ChatVisibility,
    pub chat_colors: bool,
    pub model: ModelCustomization,
    pub main_hand: SettingsMainHand,
    pub text_filtering: bool,
    pub allow_listing: bool,
}
