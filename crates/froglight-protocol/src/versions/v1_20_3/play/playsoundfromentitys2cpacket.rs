use froglight_macros::FrogReadWrite;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, FrogReadWrite)]
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
pub struct PlaySoundFromEntityS2CPacket {
    pub sound: (),
    pub category: (),
    pub entity_id: (),
    pub volume: (),
    pub pitch: (),
    pub seed: (),
}
