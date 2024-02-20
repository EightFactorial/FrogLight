//! All of the `Froglight` plugins

pub use froglight_assets::plugin::{
    AssetManagerPlugin, AssetPlugins, AssetSourcePlugin, ResourcePackPlugin, SettingsPlugin,
};
pub use froglight_core::CorePlugin;
pub use froglight_interface::plugins::{
    DebugPlugins, InterfaceLoadingScreenPlugin, InterfaceMainMenuPlugin,
    InterfaceMultiplayerMenuPlugin, InterfacePlugins, InterfaceSettingsMenuPlugin, UiScalePlugin,
};
pub use froglight_physics::PhysicsPlugin;
pub use froglight_world::WorldPlugin;

// Also re-export the plugin groups for completeness
pub use super::groups::{AppPlugins, ClientPlugins};
