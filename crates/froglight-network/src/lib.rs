#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

use bevy::app::{App, Plugin, PluginGroup, PluginGroupBuilder};

pub mod connection;
#[cfg(feature = "resolver")]
pub mod resolver;

/// The `Network` Froglight plugin group.
///
/// Adds DNS resolution and networking capabilities.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NetworkPlugins;

impl Plugin for NetworkPlugins {
    fn build(&self, app: &mut App) { <Self as PluginGroup>::build(*self).finish(app); }
}

impl PluginGroup for NetworkPlugins {
    #[allow(unused_mut)]
    fn build(self) -> PluginGroupBuilder {
        let mut builder = PluginGroupBuilder::start::<Self>();

        #[cfg(feature = "resolver")]
        {
            // Add the `ResolverPlugin` to the group.
            builder = builder.add(resolver::ResolverPlugin::default());
        }

        builder
    }
}
