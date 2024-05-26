//! System diagnostics.

use bevy::prelude::*;

mod system;
pub use system::*;

mod systemsets;
pub use systemsets::*;

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    systemsets::build(app);

    #[cfg(not(any(
        target_os = "windows",
        target_os = "linux",
        target_os = "macos",
        target_os = "android"
    )))]
    {
        warn!("System diagnostics are not supported on this platform.");
    }
    #[cfg(any(
        target_os = "windows",
        target_os = "linux",
        target_os = "macos",
        target_os = "android"
    ))]
    {
        system::build(app);
    }
}
