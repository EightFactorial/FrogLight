use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundNbtQueryResponseS2CPacket {
    pub a: u32,
    pub b: NbtCompound,
}
