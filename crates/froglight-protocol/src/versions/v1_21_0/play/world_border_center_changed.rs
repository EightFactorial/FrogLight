use froglight_macros::FrogReadWrite;

#[derive(Debug, Clone, Copy, PartialEq, FrogReadWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
pub struct WorldBorderCenterChangedPacket {
    pub center_x: f64,
    pub center_z: f64,
}
