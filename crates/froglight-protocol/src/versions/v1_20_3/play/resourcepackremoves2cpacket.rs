use froglight_macros::FrogReadWrite;
use uuid::Uuid;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, FrogReadWrite)]
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
pub struct ResourcePackRemoveS2CPacket {
    pub id: Option<Uuid>,
}
