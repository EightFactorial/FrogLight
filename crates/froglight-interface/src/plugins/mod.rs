//! Additional interface related plugins.

mod debug;
pub use debug::plugin::DebugPlugins;

pub mod uiscale;
pub use uiscale::UiScalePlugin;

mod interface;
pub use interface::InterfacePlugins;

pub mod inspector;
pub use inspector::InspectorPlugin;

mod materials;
pub use materials::MaterialPlugin;

// Re-export the menu plugins
pub use crate::menus::{
    InterfaceLoadingScreenPlugin, InterfaceMainMenuPlugin, InterfaceMultiplayerMenuPlugin,
    InterfaceSettingsMenuPlugin,
};
