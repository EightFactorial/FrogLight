use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundSelectAdvancementTabS2CPacket {
    pub a: Object,
}
