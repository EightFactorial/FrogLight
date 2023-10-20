use azalea_nbt::Nbt;
use mc_rs_macros::Transcode;

// TODO: Create a test for this packet
#[derive(Debug, Clone, PartialEq, Transcode)]
pub struct ClientboundNbtQueryResponsePacket {
    pub id: u32,
    pub nbt: Nbt,
}
