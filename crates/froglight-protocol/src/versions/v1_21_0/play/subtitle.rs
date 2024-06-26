use derive_more::{Deref, DerefMut, From, Into};
use froglight_macros::FrogReadWrite;
use simdnbt::owned::NbtTag;

#[derive(Debug, Clone, PartialEq, Deref, DerefMut, From, Into, FrogReadWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
pub struct SubtitlePacket {
    pub subtitle: NbtTag,
}
