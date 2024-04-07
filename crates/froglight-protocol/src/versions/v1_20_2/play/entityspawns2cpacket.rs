use froglight_macros::FrogReadWrite;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, FrogReadWrite)]
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
pub struct EntitySpawnS2CPacket {
    pub id: (),
    pub uuid: (),
    pub entity_type: (),
    pub x: (),
    pub y: (),
    pub z: (),
    pub pitch: (),
    pub yaw: (),
    pub head_yaw: (),
    pub entity_data: (),
    pub velocity_x: (),
    pub velocity_y: (),
    pub velocity_z: (),
}
