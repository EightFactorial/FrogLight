use bevy::prelude::*;
use froglight_assets::plugin::ResourcePackPlugin;

use super::MainMenuRootNode;

/// A plugin that adds a main menu.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct InterfaceMainMenuPlugin;

impl Plugin for InterfaceMainMenuPlugin {
    fn build(&self, app: &mut App) {
        // Add `SystemSet`s
        crate::menus::build(app);
        super::systemset::build(app);

        // Add components and systems
        super::build(app);

        // Add conditions to the `ResourcePackPlugin`
        for plugin in app.get_added_plugins::<ResourcePackPlugin>() {
            plugin.add_condition(any_with_component::<MainMenuRootNode>);
        }
    }
}
