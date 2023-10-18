use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Transcode)]
#[mctest(tests = ["transcode", "decode"], bytes = [3])]
pub enum ResourcePackAction {
    Loaded,
    Declined,
    Failed,
    Accepted,
}
