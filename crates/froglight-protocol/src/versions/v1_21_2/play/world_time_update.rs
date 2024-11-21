use froglight_macros::FrogReadWrite;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, FrogReadWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
pub struct WorldTimeUpdatePacket {
    pub world_time: u64,
    pub day_time: u64,
    pub tick_time: bool,
}
