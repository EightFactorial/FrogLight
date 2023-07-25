use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ServerboundUpdateBeaconPacket {
    pub a: Option,
    pub b: Option,
}
