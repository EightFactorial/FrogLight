use froglight_macros::FrogReadWrite;
use simdnbt::owned::Nbt;

#[derive(Debug, Clone, PartialEq, FrogReadWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
pub struct NbtQueryResponsePacket {
    #[frog(var)]
    pub id: u32,
    pub response: Nbt,
}
