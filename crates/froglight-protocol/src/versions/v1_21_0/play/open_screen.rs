use froglight_components::resourcekey::ResourceKey;
use froglight_macros::FrogReadWrite;
use simdnbt::owned::NbtTag;

#[derive(Debug, Clone, PartialEq, FrogReadWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
pub struct OpenScreenPacket {
    #[frog(var)]
    pub container_id: u32,
    pub handler: ResourceKey,
    pub name: NbtTag,
}
