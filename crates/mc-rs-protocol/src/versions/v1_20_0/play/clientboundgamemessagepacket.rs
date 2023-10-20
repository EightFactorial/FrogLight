use mc_rs_macros::Transcode;

#[derive(Debug, Clone, PartialEq, Eq, Transcode)]
#[mctest(tests = ["transcode", "decode"], bytes = [2, 77, 67, 1])]
pub struct ClientboundGameMessagePacket {
    pub message: String,
    pub overlay: bool,
}
