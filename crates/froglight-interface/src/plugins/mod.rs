//! Additional interface related plugins.

pub mod debug;
pub use debug::plugin::DebugPlugins;

pub mod uiscale;
pub use uiscale::UiScalePlugin;

mod interface;
pub use interface::InterfacePlugins;

pub mod inspector;
pub use inspector::InspectorPlugin;

// Re-export the menu plugins
pub use crate::menus::{
    InterfaceLoadingScreenPlugin, InterfaceMainMenuPlugin, InterfaceMultiplayerMenuPlugin,
    InterfaceSettingsMenuPlugin,
};
