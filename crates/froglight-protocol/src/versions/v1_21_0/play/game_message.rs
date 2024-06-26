use froglight_macros::FrogReadWrite;
use simdnbt::owned::NbtTag;

#[derive(Debug, Clone, PartialEq, FrogReadWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
pub struct GameMessagePacket {
    pub message: NbtTag,
    pub overlay: bool,
}
