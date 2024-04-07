use froglight_macros::FrogReadWrite;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, FrogReadWrite)]
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
pub struct UpdateBeaconC2SPacket {
    pub primary_effect_id: (),
    pub secondary_effect_id: (),
}
