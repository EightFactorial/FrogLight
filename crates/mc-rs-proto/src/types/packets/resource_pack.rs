use mc_rs_macros::Transcode;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Transcode)]
pub enum ResourcePackAction {
    Loaded,
    Declined,
    Failed,
    Accepted,
}
