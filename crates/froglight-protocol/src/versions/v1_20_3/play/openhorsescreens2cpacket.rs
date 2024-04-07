use froglight_macros::FrogReadWrite;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, FrogReadWrite)]
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
pub struct OpenHorseScreenS2CPacket {
    pub sync_id: (),
    pub slot_count: (),
    pub horse_id: (),
}
