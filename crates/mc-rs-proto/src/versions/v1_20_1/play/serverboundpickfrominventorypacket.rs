use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ServerboundPickFromInventoryPacket {
    pub a: u32,
}
