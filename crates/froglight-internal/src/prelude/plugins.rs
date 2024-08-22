//! Re-exports of all of the `Plugins` used in Froglight.

#[cfg(feature = "client")]
pub use froglight_asset::{
    AssetPlugin, AssetPlugins, AssetProcessorPlugin, AssetSourcePlugin, CatalogPlugin,
};
pub use froglight_block::BlockPlugin;
#[cfg(feature = "client")]
pub use froglight_interface::{
    camera::CameraPlugin,
    InterfacePlugins,
    // loading_screen::LoadingScreenPlugin,
};
pub use froglight_network::{network::NetworkPlugin, resolver::ResolverPlugin, NetworkPlugins};
#[cfg(feature = "client")]
pub use froglight_render::RenderPlugin;
pub use froglight_utils::UtilityPlugin;
