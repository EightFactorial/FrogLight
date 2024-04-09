use froglight_macros::FrogReadWrite;
use simdnbt::owned::Nbt;

#[derive(Debug, Clone, PartialEq, FrogReadWrite)]
pub struct NbtQueryResponseS2CPacket {
    pub transaction_id: u32,
    pub nbt: Nbt,
}
