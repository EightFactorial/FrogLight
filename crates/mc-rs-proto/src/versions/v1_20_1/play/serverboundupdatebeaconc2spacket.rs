use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ServerboundUpdateBeaconC2SPacket {
    pub a: Option,
    pub b: Option,
}
