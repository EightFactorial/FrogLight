use mc_rs_macros::Transcode;

#[derive(Debug, Clone, PartialEq, Eq, Transcode)]
#[mctest(tests = ["transcode", "decode"], bytes = [8, 8, 7, 6, 5, 4, 3, 2, 1, 8, 1, 2, 3, 4, 5, 6, 7, 8])]
pub struct ServerboundLoginKeyPacket {
    pub key: Vec<u8>,
    pub challenge: Vec<u8>,
}
