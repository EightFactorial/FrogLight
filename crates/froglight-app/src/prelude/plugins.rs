//! Re-exports of all of the [`Plugins`](bevy::app::Plugin) used in Froglight.

pub use froglight_core::CorePlugin;
pub use froglight_network::{
    connection::ConnectionPlugin, resolver::ResolverPlugin, NetworkPlugins,
};
pub use froglight_utils::UtilityPlugin;
