use froglight_macros::FrogReadWrite;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, FrogReadWrite)]
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
pub struct PlayerInteractEntityC2SPacket {
    pub entity_id: (),
    pub kind: (),
    pub player_sneaking: (),
}
