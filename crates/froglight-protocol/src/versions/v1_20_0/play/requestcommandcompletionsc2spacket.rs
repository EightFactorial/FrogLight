use compact_str::CompactString;
use froglight_macros::FrogReadWrite;

#[derive(Debug, Clone, PartialEq, Eq, Hash, FrogReadWrite)]
#[frog(tests = ["read_example"], bytes = [0, 4, 116, 101, 115, 116])]
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
pub struct RequestCommandCompletionsC2SPacket {
    #[frog(var)]
    pub completion_id: u32,
    pub partial_command: CompactString,
}
