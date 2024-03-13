#![doc = include_str!("README.md")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

use bevy::{app::App, log::debug};
use froglight_client::plugins::AppPlugins;

/// The global allocator.
///
/// This is completely optional, but might improve performance.
#[cfg(feature = "mimalloc")]
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

fn main() {
    // Create a new application.
    let mut app = App::new();

    // Add both the FrogLight plugins and the Bevy plugins.
    app.add_plugins(AppPlugins);

    #[cfg(feature = "mimalloc")]
    {
        // Log that we are using mimalloc.
        debug!("Using mimalloc as the global allocator");
    }
    #[cfg(not(feature = "mimalloc"))]
    {
        // Log that we are using the default allocator.
        debug!("Using the system default global allocator");
    }

    // Run the application.
    app.run();
}
