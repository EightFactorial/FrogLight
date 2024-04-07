use froglight_macros::FrogReadWrite;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, FrogReadWrite)]
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
pub struct LookAtS2CPacket {
    pub self_anchor: (),
    pub target_x: (),
    pub target_y: (),
    pub target_z: (),
    pub look_at_entity: (),
    pub entity_id: (),
    pub target_anchor: (),
}
