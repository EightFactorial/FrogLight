use froglight_macros::FrogReadWrite;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, FrogReadWrite)]
#[frog(tests = ["read_example"], bytes = [0, 0, 0, 0, 0, 0, 0, 1])]
pub struct EntityAttachS2CPacket {
    pub attached_id: u32,
    pub holding_id: i32,
}
