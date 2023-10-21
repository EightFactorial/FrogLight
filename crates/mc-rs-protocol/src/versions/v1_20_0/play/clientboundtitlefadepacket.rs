use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Transcode)]
#[mctest(tests = ["transcode", "decode"], bytes = [0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 2])]
pub struct ClientboundTitleFadePacket {
    pub fade_in: u32,
    pub stay: u32,
    pub fade_out: u32,
}
