use froglight_macros::FrogReadWrite;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, FrogReadWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
#[frog(tests = ["read_verify", "write_verify"], bytes = [0, 0])]
pub struct BundleItemSelectedPacket {
    #[frog(var)]
    pub slot_id: u32,
    #[frog(var)]
    pub item_index: u32,
}
