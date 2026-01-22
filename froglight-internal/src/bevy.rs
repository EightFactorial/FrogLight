use bevy_app::{PluginGroup, PluginGroupBuilder};

pub mod plugins {
    //! Re-exports of all provided bevy [`Plugin`](bevy_app::Plugin)s.

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
        PluginGroupBuilder::start::<Self>()
            .add(plugins::CommonPlugin)
            .add(plugins::InventoryPlugin)
            .add(plugins::WorldPlugin)
    }
}
