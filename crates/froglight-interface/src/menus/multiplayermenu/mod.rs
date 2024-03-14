//! The Multiplayer Menu
//!
//! The multiplayer menu is the screen where the
//! player can join a multiplayer game.

use bevy::{prelude::*, ui::FocusPolicy};
use froglight_assets::ResourcePackState;
use froglight_core::resources::MultiplayerMenuEnable;

use super::InterfaceMenuState;

pub(crate) mod plugin;

pub(crate) mod server_button;
// pub use server_button::MultiplayerServerButton;

pub(crate) mod server_list;
pub use server_list::MultiplayerServerListNode;

pub(crate) mod systemset;
use systemset::MultiplayerMenuUpdateSet;

#[doc(hidden)]
fn build(app: &mut App) {
    app.register_type::<MultiplayerMenuRootNode>();

    // Build the multiplayer menu
    app.add_systems(
        OnEnter(ResourcePackState::Processing),
        MultiplayerMenuRootNode::build_multimenu
            .run_if(not(any_with_component::<MultiplayerMenuRootNode>))
            .in_set(MultiplayerMenuUpdateSet),
    );

    // Show and hide the multiplayer menu
    app.add_systems(
        OnEnter(InterfaceMenuState::MultiplayerMenu),
        (
            MultiplayerMenuRootNode::show_multimenu
                .run_if(any_with_component::<MultiplayerMenuRootNode>),
            MultiplayerMenuEnable::enable,
        )
            .in_set(MultiplayerMenuUpdateSet),
    );
    app.add_systems(
        OnExit(InterfaceMenuState::MultiplayerMenu),
        (
            MultiplayerMenuRootNode::hide_multimenu
                .run_if(any_with_component::<MultiplayerMenuRootNode>),
            MultiplayerMenuEnable::disable,
        )
            .in_set(MultiplayerMenuUpdateSet),
    );

    // Handle the Escape key
    app.add_systems(
        Update,
        MultiplayerMenuRootNode::handle_escape
            .run_if(any_with_component::<MultiplayerMenuRootNode>)
            .in_set(MultiplayerMenuUpdateSet),
    );

    // Build the sub-components
    server_list::build(app);
    server_button::build(app);
}

/// A marker [`Component`] for the root [`Entity`] of the multiplayer menu.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect, Component)]
#[reflect(Component)]
pub struct MultiplayerMenuRootNode;

impl MultiplayerMenuRootNode {
    fn build_multimenu(world: &mut World) {
        debug!("Building MultiplayerMenu");

        // Determine the visibility of the multiplayer menu
        let visibility = if let Some(MultiplayerMenuEnable(true)) =
            world.get_resource::<MultiplayerMenuEnable>()
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

                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..Default::default()
            },
            background_color: BackgroundColor(Color::NONE),
            focus_policy: FocusPolicy::Block,
            visibility,
            ..Default::default()
        };

        // Spawn the root node
        let root = world.spawn((Self, root_node, Name::new("MultiplayerMenuRootNode"))).id();

        // Build the multiplayer server list
        MultiplayerServerListNode::build(world, root);
    }

    /// Shows the multiplayer menu.
    fn show_multimenu(mut query: Query<&mut Visibility, With<Self>>) {
        debug!("Showing MultiplayerMenu");
        for mut vis in &mut query {
            *vis = Visibility::Inherited;
        }
    }

    /// Hides the multiplayer menu.
    fn hide_multimenu(mut query: Query<&mut Visibility, With<Self>>) {
        debug!("Hiding MultiplayerMenu");
        for mut vis in &mut query {
            *vis = Visibility::Hidden;
        }
    }

    /// Handles the Escape key.
    fn handle_escape(
        input: Res<ButtonInput<KeyCode>>,
        mut state: ResMut<NextState<InterfaceMenuState>>,
    ) {
        if input.just_pressed(KeyCode::Escape) {
            debug!("Pressed Escape");
            state.set(InterfaceMenuState::MainMenu);
        }
    }
}
