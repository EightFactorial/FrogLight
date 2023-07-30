use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Transcode)]
#[bitset]
pub struct InputFlags {
    pub jumping: bool,
    pub shift: bool,
}
