use bevy::{app::PluginGroupBuilder, prelude::*};

/// A [`PluginGroup`] for debug related plugins.
///
/// Can be used to add all debug related plugins to an [`App`].
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DebugPlugins;

impl PluginGroup for DebugPlugins {
    fn build(self) -> PluginGroupBuilder { PluginGroupBuilder::start::<Self>() }
}

impl Plugin for DebugPlugins {
    fn build(&self, app: &mut App) {
        // Add `SystemSet`s
        super::systemset::build(app);

        // Add `Self` as a plugin group
        <Self as PluginGroup>::build(Self).finish(app);
    }
}
