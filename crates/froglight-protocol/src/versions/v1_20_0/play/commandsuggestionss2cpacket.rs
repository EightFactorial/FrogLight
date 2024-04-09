use froglight_macros::FrogReadWrite;

use crate::common::UnsizedBuffer;

#[derive(Debug, Clone, PartialEq, Eq, Hash, FrogReadWrite)]
pub struct CommandSuggestionsS2CPacket {
    #[frog(var)]
    pub id: i32,
    // TODO: Implement CommandSuggestion
    pub suggestions: UnsizedBuffer,
}
