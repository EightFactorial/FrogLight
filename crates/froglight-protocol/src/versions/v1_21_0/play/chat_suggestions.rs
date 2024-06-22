use compact_str::CompactString;
use froglight_macros::FrogReadWrite;

use crate::packet::ChatSuggestionAction;

#[derive(Debug, Clone, PartialEq, Eq, Hash, FrogReadWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
#[frog(tests = ["read_example"], bytes = [0, 1, 2, 77, 67])]
pub struct ChatSuggestionsPacket {
    pub action: ChatSuggestionAction,
    pub entries: Vec<CompactString>,
}
