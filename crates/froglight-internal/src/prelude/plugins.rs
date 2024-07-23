//! Re-exports of all of the `Plugins` used in Froglight.

#[cfg(feature = "client")]
pub use froglight_asset::{
    AssetDefinitionPlugin, AssetLoaderPlugin, AssetPlugins, AssetProcessorPlugin,
    AssetSourcePlugin, AssetStoragePlugin,
};
pub use froglight_block::BlockPlugin;
#[cfg(feature = "client")]
pub use froglight_interface::{loading_screen::LoadingScreenPlugin, InterfacePlugins};
pub use froglight_network::{network::NetworkPlugin, resolver::ResolverPlugin, NetworkPlugins};
pub use froglight_utils::UtilityPlugin;
