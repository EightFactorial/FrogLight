use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ServerboundPickFromInventoryC2SPacket {
    pub a: u32,
}
