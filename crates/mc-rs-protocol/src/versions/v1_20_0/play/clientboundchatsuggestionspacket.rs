use mc_rs_macros::Transcode;

use crate::types::packets::suggestion::SuggestionAction;

#[derive(Debug, Clone, PartialEq, Eq, Transcode)]
#[mctest(tests = ["transcode", "decode"], bytes = [0, 1, 2, 77, 67])]
pub struct ClientboundChatSuggestionsPacket {
    pub suggestion: SuggestionAction,
    pub entries: Vec<String>,
}
