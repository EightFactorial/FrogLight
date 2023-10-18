use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Transcode)]
#[mctest(tests = ["transcode", "decode"], bytes = [0, 0, 0, 0, 64])]
pub enum ScoreboardUpdate {
    Change(u32),
    Remove,
}
