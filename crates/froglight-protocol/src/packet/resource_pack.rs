#![allow(missing_docs)]

use froglight_macros::FrogReadWrite;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, FrogReadWrite)]
#[frog(tests = ["read_example"], bytes = [0])]
pub enum ResourcePackAction {
    SuccessfullyLoaded,
    Declined,
    FailedDownload,
    Accepted,
    InvalidUrl,
    FailedReload,
    Discarded,
}
