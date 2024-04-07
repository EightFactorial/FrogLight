use froglight_macros::FrogReadWrite;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, FrogReadWrite)]
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
pub struct BlockEntityUpdateS2CPacket {
    pub pos: (),
    pub block_entity_type: (),
    pub nbt: (),
}
