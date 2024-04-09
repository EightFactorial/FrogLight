use froglight_macros::FrogReadWrite;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, FrogReadWrite)]
#[frog(tests = ["read_example"], bytes = [28, 0, 0, 1, 231])]
pub struct EntityStatusS2CPacket {
    pub entity_id: u32,
    pub status: u8,
}
