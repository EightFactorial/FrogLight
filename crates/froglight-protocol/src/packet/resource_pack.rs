#![allow(missing_docs)]

use froglight_macros::FrogReadWrite;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, FrogReadWrite)]
#[frog(tests = ["read_example"], bytes = [0])]
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
pub enum ResourcePackAction {
    SuccessfullyLoaded,
    Declined,
    FailedDownload,
    Accepted,
    InvalidUrl,
    FailedReload,
    Discarded,
}
