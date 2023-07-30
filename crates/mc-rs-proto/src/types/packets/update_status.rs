use mc_rs_macros::Transcode;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Transcode)]
pub enum StatusAction {
    PerformRespawn,
    RequestStats,
}
