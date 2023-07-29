use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Transcode)]
pub enum ScoreboardUpdate {
    Change(u32),
    Remove,
}
