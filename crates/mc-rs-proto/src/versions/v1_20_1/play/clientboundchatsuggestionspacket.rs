use mc_rs_macros::Transcode;

use crate::types::packets::suggestion::SuggestionAction;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundChatSuggestionsPacket {
    pub suggestion: SuggestionAction,
    pub entries: Vec<String>,
}
