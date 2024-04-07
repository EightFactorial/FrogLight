use froglight_macros::FrogReadWrite;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, FrogReadWrite)]
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
pub struct ClickSlotC2SPacket {
    pub field_0: (),
    pub field_1: (),
    pub field_2: (),
    pub field_3: (),
    pub field_4: (),
    pub field_5: (),
    pub field_6: (),
}
