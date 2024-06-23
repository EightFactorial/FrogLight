use froglight_macros::FrogReadWrite;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, FrogReadWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
#[frog(tests = ["read_example"], bytes = [1, 0, 1, 0])]
pub struct UpdateBeaconPacket {
    #[frog(var)]
    pub primary_effect: Option<u32>,
    #[frog(var)]
    pub secondary_effect: Option<u32>,
}
