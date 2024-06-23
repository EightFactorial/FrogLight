use froglight_macros::FrogReadWrite;

#[derive(Debug, Clone, Copy, PartialEq, FrogReadWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
pub struct UpdateTickRatePacket {
    pub tick_rate: f32,
    pub frozen: bool,
}
