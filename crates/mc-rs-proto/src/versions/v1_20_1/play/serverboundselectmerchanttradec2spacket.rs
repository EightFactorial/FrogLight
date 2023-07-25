use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ServerboundSelectMerchantTradeC2SPacket {
    pub a: u32,
}
