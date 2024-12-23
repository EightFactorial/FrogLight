//! Re-exports of all of the `Plugins` used in Froglight.

pub use froglight_block::BlockPlugin;
pub use froglight_entity::EntityPlugin;
pub use froglight_network::{network::NetworkPlugin, resolver::ResolverPlugin, NetworkPlugins};
#[cfg(feature = "reflect")]
pub use froglight_registry::RegistryPlugin;
pub use froglight_utils::UtilityPlugin;
