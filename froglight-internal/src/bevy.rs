//! TODO

pub mod plugins {
    //! Re-exports of all provided bevy [`Plugin`](bevy_app::Plugin)s.

    #[cfg(feature = "std")]
    pub use crate::modules::tick::diagnostic::TickMeasurementPlugin;
    #[cfg(feature = "network")]
    pub use crate::modules::{api::bevy::ApiPlugin, network::bevy::NetworkPlugin};
    pub use crate::{
        bevy::FroglightPlugins,
        modules::{
            brigadier::bevy::BrigadierPlugin, entity::bevy::EntityPlugin,
            instance::bevy::InstancePlugin, inventory::bevy::InventoryPlugin,
            physics::bevy::PhysicsPlugin, tick::bevy::TickPlugin, world::bevy::WorldPlugin,
        },
    };
}

// -------------------------------------------------------------------------------------------------

#[allow(clippy::wildcard_imports, reason = "Ignored")]
use plugins::*;

bevy_app::plugin_group! {
    /// A [`PluginGroup`] that includes all of `froglight`'s bevy plugins.
    #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct FroglightPlugins {
        #[cfg(feature = "network")]
        :ApiPlugin,
        #[cfg(feature = "network")]
        :NetworkPlugin,
        :BrigadierPlugin,
        :EntityPlugin,
        :InstancePlugin,
        :InventoryPlugin,
        :PhysicsPlugin,
        :TickPlugin,
        #[cfg(feature = "std")]
        :TickMeasurementPlugin,
        :WorldPlugin
    }
    /// B
}
