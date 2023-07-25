use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundSelectAdvancementTabPacket {
    pub a: Object,
}
