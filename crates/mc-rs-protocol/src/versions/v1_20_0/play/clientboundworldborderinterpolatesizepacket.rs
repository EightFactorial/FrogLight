use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Copy, PartialEq, Transcode)]
#[mctest(tests = ["transcode", "decode"], bytes = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])]
pub struct ClientboundWorldBorderInterpolateSizePacket {
    pub old_size: f64,
    pub new_size: f64,
    #[var]
    pub time: u64,
}
