use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ServerboundUpdateBeaconPacket {
    #[var]
    pub primary: Option<u32>,
    #[var]
    pub secondary: Option<u32>,
}
