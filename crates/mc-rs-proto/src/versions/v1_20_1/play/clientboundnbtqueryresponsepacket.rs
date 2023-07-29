use fastnbt::Value;
use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundNbtQueryResponsePacket {
    pub id: u32,
    pub nbt: Value,
}
