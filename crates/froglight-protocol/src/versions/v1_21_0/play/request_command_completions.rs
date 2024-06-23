use compact_str::CompactString;
use froglight_macros::FrogReadWrite;

#[derive(Debug, Clone, PartialEq, Eq, Hash, FrogReadWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
pub struct RequestCommandCompletionsPacket {
    #[frog(var)]
    pub id: u32,
    pub command: CompactString,
}
