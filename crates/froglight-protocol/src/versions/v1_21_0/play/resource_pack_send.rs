use compact_str::CompactString;
use froglight_macros::FrogReadWrite;
use simdnbt::owned::NbtTag;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, FrogReadWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
pub struct ResourcePackSendPacket {
    pub uuid: Uuid,
    pub url: CompactString,
    pub hash: CompactString,
    pub required: bool,
    pub prompt: Option<NbtTag>,
}
