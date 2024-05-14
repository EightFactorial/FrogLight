//! Re-exports of all of the [`Plugins`](bevy::app::Plugin) used in Froglight.

#[cfg(feature = "bevy_asset")]
pub use froglight_assets::AssetPlugin;
#[cfg(feature = "bevy_asset")]
pub use froglight_client::ClientPlugin;
pub use froglight_network::{
    connection::ConnectionPlugin, resolver::ResolverPlugin, NetworkPlugins,
};
pub use froglight_registry::RegistryPlugin;
#[cfg(feature = "bevy_asset")]
pub use froglight_render::RenderPlugin;
pub use froglight_settings::SettingsPlugin;
pub use froglight_utils::UtilityPlugin;
