use froglight_macros::FrogReadWrite;

use crate::common::{EntityUuid, UnsizedBuffer};

#[derive(Debug, Clone, PartialEq, Eq, FrogReadWrite)]
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
pub struct BossBarS2CPacket {
    pub uuid: EntityUuid,
    // TODO: Implement BossBar
    pub action: UnsizedBuffer,
}
