//! The Main Menu
//!
//! The main menu is the first screen the player sees when starting the game.

use bevy::{prelude::*, ui::FocusPolicy};
use froglight_assets::ResourcePackState;

pub(crate) mod plugin;

pub(crate) mod background;
pub use background::{MainMenuBackground, MainMenuBackgroundCamera};

pub(crate) mod buttons;
pub use buttons::{
    MainMenuButtonNode, MainMenuMultiplayerButton, MainMenuQuitButton, MainMenuSettingsButton,
};

pub(crate) mod logo;
pub use logo::{MainMenuLogo, MainMenuLogoNode, MainMenuSubLogo};

pub(crate) mod splash;
pub use splash::MainMenuSplashText;

pub(crate) mod systemset;
use systemset::MainMenuUpdateSet;

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

    background::build(app);
    buttons::build(app);
    logo::build(app);
    splash::build(app);
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

                justify_content: JustifyContent::Center,
                ..Default::default()
            },
            background_color: BackgroundColor(Color::NONE),
            focus_policy: FocusPolicy::Block,
            ..Default::default()
        };

        // Spawn the root node
        let root = world.spawn((Self, root_node, Name::new("MainMenuRootNode"))).id();

        // Build children
        MainMenuButtonNode::build(world, root);
        MainMenuLogoNode::build(world, root);

        // Build the background
        MainMenuBackground::build(world);
    }
}
