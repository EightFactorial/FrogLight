//! Various [`PluginGroup`](bevy::prelude::PluginGroup)s used in `Froglight`

mod app;
pub use app::AppPlugins;

mod client;
pub use client::ClientPlugins;
