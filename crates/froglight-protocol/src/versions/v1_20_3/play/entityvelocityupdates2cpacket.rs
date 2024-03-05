use froglight_macros::FrogReadWrite;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, FrogReadWrite)]
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
pub struct EntityVelocityUpdateS2CPacket {
    pub id: (),
    pub velocity_x: (),
    pub velocity_y: (),
    pub velocity_z: (),
}
