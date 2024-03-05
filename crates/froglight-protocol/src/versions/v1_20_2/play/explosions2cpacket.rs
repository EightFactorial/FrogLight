use froglight_macros::FrogReadWrite;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, FrogReadWrite)]
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
pub struct ExplosionS2CPacket {
    pub x: (),
    pub y: (),
    pub z: (),
    pub radius: (),
    pub affected_blocks: (),
    pub player_velocity_x: (),
    pub player_velocity_y: (),
    pub player_velocity_z: (),
}
