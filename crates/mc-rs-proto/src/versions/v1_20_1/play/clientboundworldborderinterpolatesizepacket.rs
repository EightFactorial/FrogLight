use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Transcode)]
pub struct ClientboundWorldBorderInterpolateSizePacket {
    pub old_size: f64,
    pub new_size: f64,
    #[var]
    pub time: u64,
}
