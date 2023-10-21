use mc_rs_macros::Transcode;

#[derive(Debug, Clone, PartialEq, Eq, Transcode)]
#[mctest(tests = ["transcode", "decode"], bytes = [0, 4, 84, 101, 115, 116])]
pub struct ClientboundScoreboardDisplayPacket {
    pub slot: u8,
    pub name: String,
}
