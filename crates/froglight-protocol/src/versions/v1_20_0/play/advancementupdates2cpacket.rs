use froglight_macros::FrogReadWrite;

use crate::common::UnsizedByteBuffer;

#[derive(Debug, Clone, PartialEq, Eq, Hash, FrogReadWrite)]
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
pub struct AdvancementUpdateS2CPacket {
    pub reset: bool,
    // TODO: Implement advancements
    pub data: UnsizedByteBuffer,
    // pub added: HashMap<ResourceLocation, Advancement>,
    // pub removed: Vec<ResourceLocation>,
    // pub progress: HashMap<ResourceLocation, AdvancementProgress>,
}
