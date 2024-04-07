use froglight_macros::FrogReadWrite;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, FrogReadWrite)]
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
pub struct ScreenHandlerSlotUpdateS2CPacket {
    pub sync_id: (),
    pub revision: (),
    pub slot: (),
    pub stack: (),
}
