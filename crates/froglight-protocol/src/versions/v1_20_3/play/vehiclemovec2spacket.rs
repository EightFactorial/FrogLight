use froglight_macros::FrogReadWrite;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, FrogReadWrite)]
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
pub struct VehicleMoveC2SPacket {
    pub x: (),
    pub y: (),
    pub z: (),
    pub yaw: (),
    pub pitch: (),
}
