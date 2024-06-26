use froglight_macros::FrogReadWrite;
use simdnbt::owned::NbtTag;

#[derive(Debug, Clone, PartialEq, FrogReadWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
pub struct ServerMetadataPacket {
    pub metadata: NbtTag,
    pub favicon: Option<Vec<u8>>,
}
