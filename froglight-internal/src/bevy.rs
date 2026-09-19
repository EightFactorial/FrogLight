//! TODO

use bevy_app::{PluginGroup, PluginGroupBuilder};

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

/// A [`PluginGroup`] that includes all of froglight's bevy
/// [`Plugin`](bevy_app::Plugin)s.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FroglightPlugins;

impl PluginGroup for FroglightPlugins {
    #[allow(unused_mut, reason = "Used if features are enabled")]
    fn build(self) -> PluginGroupBuilder {
        let mut group = PluginGroupBuilder::start::<Self>();

        #[cfg(feature = "network")]
        {
            group = group.add(plugins::ApiPlugin).add(plugins::NetworkPlugin);
        }

        group
            .add(plugins::BrigadierPlugin)
            .add(plugins::EntityPlugin)
            .add(plugins::InstancePlugin)
            .add(plugins::InventoryPlugin)
            .add(plugins::PhysicsPlugin)
            .add(plugins::TickPlugin)
            .add(plugins::WorldPlugin)
    }
}
