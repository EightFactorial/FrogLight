use froglight_macros::FrogReadWrite;

/// The status of a resource pack.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, FrogReadWrite)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
#[frog(tests = ["read_example"], bytes = [0])]
pub enum ResourcePackStatus {
    /// The resource pack was successfully loaded.
    SuccessfullyLoaded,
    /// The resource pack was declined.
    Declined,
    /// The resource pack failed to download.
    FailedDownload,
    /// The resource pack was accepted.
    Accepted,
    /// The resource pack url was invalid.
    InvalidUrl,
    /// The resource pack was failed to reload.
    FailedReload,
    /// The resource pack was discarded.
    Discarded,
}
