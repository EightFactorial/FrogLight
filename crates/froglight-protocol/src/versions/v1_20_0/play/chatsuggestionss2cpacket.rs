use compact_str::CompactString;
use froglight_macros::FrogReadWrite;

use crate::common::ChatSuggestionAction;

#[derive(Debug, Clone, PartialEq, Eq, Hash, FrogReadWrite)]
#[frog(tests = ["read_example"], bytes = [0, 1, 2, 77, 67])]
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
pub struct ChatSuggestionsS2CPacket {
    pub action: ChatSuggestionAction,
    pub entries: Vec<CompactString>,
}
