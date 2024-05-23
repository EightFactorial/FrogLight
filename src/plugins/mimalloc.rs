use bevy::app::{App, Plugin};
use mimalloc::MiMalloc;

/// The mimalloc global allocator.
///
/// This is optional, but likely improves performance.
#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

/// A plugin for logging the version of [`MiMalloc`].
///
/// This plugin does not enable [`MiMalloc`], it is
/// automatically enabled with the `mimalloc` feature.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MiMallocPlugin;

impl Plugin for MiMallocPlugin {
    fn build(&self, _: &mut App) {
        bevy::log::info!("Using MiMalloc v{}", GLOBAL.version());
    }
}
