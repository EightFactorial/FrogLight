use froglight_macros::FrogReadWrite;

use crate::common::UnsizedByteBuffer;

#[derive(Debug, Clone, PartialEq, Eq, Hash, FrogReadWrite)]
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
pub struct CommandSuggestionsS2CPacket {
    #[frog(var)]
    pub id: i32,
    // TODO: Implement CommandSuggestion
    pub suggestions: UnsizedByteBuffer,
}
