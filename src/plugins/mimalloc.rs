use bevy::app::{App, Plugin};
use mimalloc::MiMalloc;

/// The global allocator.
///
/// This is completely optional, but might improve performance.
#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

/// The MiMalloc plugin.
///
/// Logs the version of [`MiMalloc`] to the console.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MiMallocPlugin;

impl Plugin for MiMallocPlugin {
    fn build(&self, _: &mut App) {
        bevy::log::info!("Using MiMalloc v{}", GLOBAL.version());
    }
}
