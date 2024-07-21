use froglight_common::EntityUuid;
use froglight_macros::FrogReadWrite;

use crate::common::UnsizedBuffer;

#[derive(Debug, Clone, PartialEq, Eq, Hash, FrogReadWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
pub struct BossBarPacket {
    pub entity_uuid: EntityUuid,
    // TODO: Implement BossBars
    pub action: UnsizedBuffer,
}
