use froglight_macros::FrogReadWrite;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, FrogReadWrite)]
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
pub struct ClientCommandC2SPacket {
    pub entity_id: (),
    pub mode: (),
    pub mount_jump_height: (),
}
