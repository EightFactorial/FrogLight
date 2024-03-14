//! The Main Menu
//!
//! The main menu is the first screen the player sees when starting the game.

use bevy::{prelude::*, ui::FocusPolicy};
use froglight_assets::ResourcePackState;

pub(crate) mod plugin;

pub(crate) mod buttons;
pub use buttons::{
    MainMenuButtonNode, MainMenuMultiplayerButton, MainMenuMultiplayerButtonEvent,
    MainMenuQuitButton, MainMenuQuitButtonEvent, MainMenuSettingsButton,
    MainMenuSettingsButtonEvent,
};

pub(crate) mod logo;
use froglight_core::resources::MainMenuEnable;
pub use logo::{MainMenuLogo, MainMenuLogoNode, MainMenuSubLogo};

pub(crate) mod splash;
pub use splash::MainMenuSplashText;

pub(crate) mod systemset;
use systemset::MainMenuUpdateSet;

pub(crate) mod text;
pub use text::{MainMenuNoticeText, MainMenuVersionText};

use super::InterfaceMenuState;

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

    // Show and hide the main menu
    app.add_systems(
        OnEnter(InterfaceMenuState::MainMenu),
        (
            MainMenuRootNode::show_mainmenu.run_if(any_with_component::<MainMenuRootNode>),
            MainMenuEnable::enable,
        )
            .in_set(MainMenuUpdateSet),
    );
    app.add_systems(
        OnExit(InterfaceMenuState::MainMenu),
        (
            MainMenuRootNode::hide_mainmenu.run_if(any_with_component::<MainMenuRootNode>),
            MainMenuEnable::disable,
        )
            .in_set(MainMenuUpdateSet),
    );

    // Build the sub-components
    buttons::build(app);
    logo::build(app);
    splash::build(app);
    text::build(app);
}

/// A marker [`Component`] for the root [`Entity`] of the main menu.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect, Component)]
#[reflect(Component)]
pub struct MainMenuRootNode;

impl MainMenuRootNode {
    fn build_mainmenu(world: &mut World) {
        debug!("Building MainMenu");

        // Determine the visibility of the main menu
        let visibility = if let Some(MainMenuEnable(true)) = world.get_resource::<MainMenuEnable>()
        {
            Visibility::Inherited
        } else {
            Visibility::Hidden
        };

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
            visibility,
            ..Default::default()
        };

        // Spawn the root node
        let root = world.spawn((Self, root_node, Name::new("MainMenuRootNode"))).id();

        // Build children
        MainMenuLogoNode::build(world, root);
        MainMenuButtonNode::build(world, root);
        MainMenuVersionText::build(world, root);
        MainMenuNoticeText::build(world, root);
    }

    /// Shows the main menu.
    fn show_mainmenu(mut query: Query<&mut Visibility, With<Self>>) {
        debug!("Showing MainMenuRootNode");
        for mut vis in &mut query {
            *vis = Visibility::Inherited;
        }
    }

    /// Hides the main menu.
    fn hide_mainmenu(mut query: Query<&mut Visibility, With<Self>>) {
        debug!("Hiding MainMenuRootNode");
        for mut vis in &mut query {
            *vis = Visibility::Hidden;
        }
    }
}
