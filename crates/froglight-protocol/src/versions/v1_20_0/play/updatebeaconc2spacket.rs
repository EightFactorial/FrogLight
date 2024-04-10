use froglight_macros::FrogReadWrite;

// TODO: Potion Ids
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, FrogReadWrite)]
#[frog(tests = ["read_example"], bytes = [1, 0, 1, 0])]
pub struct UpdateBeaconC2SPacket {
    #[frog(var)]
    pub primary_effect_id: Option<u32>,
    #[frog(var)]
    pub secondary_effect_id: Option<u32>,
}
