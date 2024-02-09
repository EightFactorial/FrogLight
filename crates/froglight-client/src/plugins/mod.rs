//! Client plugins for [`FrogLight`](crate).
//!
//! There are two [`PluginGroups`](bevy::app::PluginGroup) available:
//! - [`AppPlugins`] - Both `FrogLight` plugins and [`bevy's`](bevy)
//!   [`DefaultPlugins`](bevy::DefaultPlugins) plugins.
//! - [`ClientPlugins`] - Only `FrogLight` plugins.
//!
//! The [`AppPlugins`] group is recommended for most use cases.
//!
//! Use [`ClientPlugins`] if you want to manage bevy's plugins yourself.
//!
//! # Example
//!
//! ```rust,no_run,
//! use bevy::app::App;
//! use froglight_client::plugins::AppPlugins;
//!
//! // Create a new application.
//! let mut app = App::new();
//!
//! // Add both the FrogLight plugins and the Bevy plugins.
//! app.add_plugins(AppPlugins);
//!
//! // Run the application.
//! app.run();
//! ```

// Re-export plugins

pub use froglight_debug::DebugPlugin;
#[cfg(feature = "default-loading")]
pub use froglight_loading::LoadingPlugin;
pub use froglight_resourcepack::ResourcePackPlugin;
pub use froglight_settings::SettingsPlugin;
pub use froglight_world::WorldPlugin;

// Export plugin groups

mod groups;
pub use groups::{app::AppPlugins, client::ClientPlugins};
