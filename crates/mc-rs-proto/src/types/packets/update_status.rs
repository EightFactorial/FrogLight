use mc_rs_macros::Transcode;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Transcode)]
#[mctest(tests = ["transcode", "decode"], bytes = [0])]
pub enum StatusAction {
    PerformRespawn,
    RequestStats,
}
