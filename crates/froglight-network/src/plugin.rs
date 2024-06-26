use bevy_app::{App, Plugin, PluginGroup, PluginGroupBuilder};

/// The `Network` Froglight plugin group.
///
/// Adds DNS resolution and networking capabilities.
///
/// Adds the following plugins:
/// - [`NetworkPlugin`](crate::NetworkPlugin)
/// - [`ResolverPlugin`](crate::ResolverPlugin) (if the `resolver` feature is
///   enabled)
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NetworkPlugins;

impl NetworkPlugins {
    /// Returns the [`NetworkPlugins`] group as a [`Plugin`].
    #[must_use]
    pub fn as_plugin(self) -> impl Plugin { self }

    /// Returns the [`NetworkPlugins`] group as a [`PluginGroup`].
    #[must_use]
    pub fn as_plugingroup(self) -> impl PluginGroup { self }
}

impl Plugin for NetworkPlugins {
    fn build(&self, app: &mut App) { <Self as PluginGroup>::build(*self).finish(app); }
}

impl PluginGroup for NetworkPlugins {
    #[allow(unused_mut)]
    fn build(self) -> PluginGroupBuilder {
        let mut builder = PluginGroupBuilder::start::<Self>().add(crate::NetworkPlugin);

        #[cfg(feature = "resolver")]
        {
            // Add the `ResolverPlugin` to the group.
            builder = builder.add(crate::ResolverPlugin::default());
        }

        builder
    }
}
