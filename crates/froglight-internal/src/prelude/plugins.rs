//! Re-exports of all of the `Plugins` used in Froglight.

pub use froglight_network::{
    connection::ConnectionPlugin, resolver::ResolverPlugin, NetworkPlugins,
};
pub use froglight_registry::RegistryPlugin;
pub use froglight_utils::UtilityPlugin;
