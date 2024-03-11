use bevy::{app::PluginGroupBuilder, prelude::*};

use super::{InspectorPlugin, MaterialPlugin, UiScalePlugin};
use crate::menus::{
    InterfaceLoadingScreenPlugin, InterfaceMainMenuPlugin, InterfaceMultiplayerMenuPlugin,
    InterfacePanoramaPlugin, InterfaceSettingsMenuPlugin,
};

/// A [`PluginGroup`] for interface related plugins.
///
/// Can be used to add all interface related plugins to an [`App`].
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct InterfacePlugins;

impl PluginGroup for InterfacePlugins {
    fn build(self) -> PluginGroupBuilder {
        let mut group = PluginGroupBuilder::start::<Self>();

        // Add miscellaneous plugins
        group = group.add(UiScalePlugin);

        #[cfg(feature = "inspector")]
        {
            group = group.add(InspectorPlugin);
        }

        // Add menu plugins
        group
            .add(InterfaceLoadingScreenPlugin)
            .add(InterfacePanoramaPlugin)
            .add(InterfaceMainMenuPlugin)
            .add(InterfaceMultiplayerMenuPlugin)
            .add(InterfaceSettingsMenuPlugin)
            .add(MaterialPlugin)
    }
}

impl Plugin for InterfacePlugins {
    fn build(&self, app: &mut App) {
        // Add `Self` as a plugin group
        <Self as PluginGroup>::build(Self).finish(app);
    }
}
