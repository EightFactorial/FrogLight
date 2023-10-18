use mc_rs_macros::Transcode;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Transcode)]
#[mctest(tests = ["transcode", "encode", "decode"], bytes = [0])]
#[bitset]
pub struct InputFlags {
    pub jumping: bool,
    pub shift: bool,
}
