use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Transcode)]
#[mctest(tests = ["transcode", "decode"], bytes = [1, 0, 1, 0])]
pub struct ServerboundUpdateBeaconPacket {
    // TODO: Potion ID
    #[var]
    pub primary: Option<u32>,
    // TODO: Potion ID
    #[var]
    pub secondary: Option<u32>,
}
