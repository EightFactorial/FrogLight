//! The Main Menu
//!
//! The main menu is the first screen the player sees when starting the game.

use bevy::prelude::*;
use froglight_assets::ResourcePackState;

use self::systemset::MainMenuUpdateSet;

pub(crate) mod plugin;
pub(crate) mod systemset;

#[doc(hidden)]
fn build(app: &mut App) {
    app.register_type::<MainMenuRootNode>();

    // Build the main menu
    app.add_systems(
        OnEnter(ResourcePackState::Processing),
        MainMenuRootNode::build_mainmenu
            .run_if(not(any_with_component::<MainMenuRootNode>))
            .in_set(MainMenuUpdateSet),
    );
}

/// A marker [`Component`] for the root [`Entity`] of the main menu.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect, Component)]
#[reflect(Component)]
pub struct MainMenuRootNode;

impl MainMenuRootNode {
    fn build_mainmenu(world: &mut World) {
        debug!("Building MainMenu");

        // Create the root node
        let root_node = NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,

                width: Val::Percent(100.0),
                height: Val::Percent(100.0),

                ..Default::default()
            },
            background_color: BackgroundColor(Color::NONE),
            ..Default::default()
        };

        // Spawn the root node
        world.spawn((Self, root_node, Name::new("MainMenuRootNode")));

        // TODO: Add the main menu UI
    }
}
