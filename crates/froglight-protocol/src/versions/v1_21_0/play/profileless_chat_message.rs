use froglight_macros::FrogReadWrite;
use simdnbt::owned::NbtTag;

#[derive(Debug, Clone, PartialEq, FrogReadWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
pub struct ProfilelessChatMessagePacket {
    pub message: NbtTag,
    #[frog(var)]
    pub chat_type: u32,
    pub sender: NbtTag,
    pub target: Option<NbtTag>,
}
