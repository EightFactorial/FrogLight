use bevy_app::{PluginGroup, PluginGroupBuilder};

pub mod plugins {
    //! Re-exports of all provided bevy [`Plugin`](bevy_app::Plugin)s.

    #[cfg(feature = "network")]
    pub use crate::{api::bevy::ApiPlugin, network::bevy::NetworkPlugin};
    pub use crate::{
        bevy::FroglightPlugins, common::bevy::CommonPlugin, inventory::bevy::InventoryPlugin,
        world::bevy::WorldPlugin,
    };
}

/// A [`PluginGroup`] that includes all of froglight's bevy
/// [`Plugin`](bevy_app::Plugin)s.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FroglightPlugins;

impl PluginGroup for FroglightPlugins {
    fn build(self) -> PluginGroupBuilder {
        let mut group = PluginGroupBuilder::start::<Self>();

        #[cfg(feature = "network")]
        {
            group = group.add(plugins::ApiPlugin).add(plugins::NetworkPlugin);
        }

        group.add(plugins::CommonPlugin).add(plugins::InventoryPlugin).add(plugins::WorldPlugin)
    }
}
