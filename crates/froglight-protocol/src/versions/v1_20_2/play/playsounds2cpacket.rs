use froglight_macros::FrogReadWrite;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, FrogReadWrite)]
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
pub struct PlaySoundS2CPacket {
    pub sound: (),
    pub category: (),
    pub fixed_x: (),
    pub fixed_y: (),
    pub fixed_z: (),
    pub volume: (),
    pub pitch: (),
    pub seed: (),
}
