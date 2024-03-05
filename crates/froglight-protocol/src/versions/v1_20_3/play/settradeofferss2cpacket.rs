use froglight_macros::FrogReadWrite;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, FrogReadWrite)]
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
pub struct SetTradeOffersS2CPacket {
    pub sync_id: (),
    pub level_progress: (),
    pub experience: (),
    pub leveled: (),
    pub refreshable: (),
}
