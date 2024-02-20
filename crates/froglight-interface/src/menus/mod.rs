//! Menu plugins

pub mod loadingscreen;
pub use loadingscreen::plugin::InterfaceLoadingScreenPlugin;

pub mod mainmenu;
pub use mainmenu::plugin::InterfaceMainMenuPlugin;

pub mod multiplayermenu;
pub use multiplayermenu::plugin::InterfaceMultiplayerMenuPlugin;

pub mod settingsmenu;
pub use settingsmenu::plugin::InterfaceSettingsMenuPlugin;
