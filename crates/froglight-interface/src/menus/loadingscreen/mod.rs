//! The screen displayed while loading resources.
//!
//! Appears while the game is starting up or reloading resource packs.

use bevy::prelude::*;
use froglight_core::events::ResourcePackStartLoadingEvent;

pub(crate) mod plugin;

mod bar;
pub use bar::{ProgressBar, ProgressBarNode};

mod logo;
pub use logo::{LoadingScreenLogo, LoadingScreenLogoNode};

pub(crate) mod systemset;
use systemset::{LoadingScreenPostStartupSet, LoadingScreenStateSet};

#[doc(hidden)]
fn build(app: &mut App) {
    app.register_type::<LoadingScreenRootNode>();

    // Build the loading screen on startup
    app.add_systems(
        PostStartup,
        LoadingScreenRootNode::build_loadingscreen
            .run_if(run_once())
            .in_set(LoadingScreenPostStartupSet),
    );

    // Show/hide the loading screen
    app.add_systems(
        OnEnter(LoadingScreenStateSet::Shown),
        LoadingScreenRootNode::show_loadingscreen.in_set(LoadingScreenStateSet::Shown),
    );
    app.add_systems(
        OnEnter(LoadingScreenStateSet::Hidden),
        LoadingScreenRootNode::hide_loadingscreen.in_set(LoadingScreenStateSet::Hidden),
    );

    // Show the loading screen when resource packs are being reloaded
    app.add_systems(
        Update,
        LoadingScreenRootNode::listen_for_resourcepacks
            .ambiguous_with_all()
            .run_if(on_event::<ResourcePackStartLoadingEvent>())
            .in_set(LoadingScreenStateSet::Hidden),
    );

    // Add child modules
    bar::build(app);
    logo::build(app);
}

/// A marker [`Component`] for the root [`Entity`] of the loading screen.
///
/// All UI elements of the loading screen should be children of this entity.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect, Component)]
#[reflect(Component)]
pub struct LoadingScreenRootNode;

impl LoadingScreenRootNode {
    /// Builds the loading screen.
    fn build_loadingscreen(world: &mut World) {
        debug!("Building LoadingScreen");

        // Create the root entity
        let mut background_node = center_node();
        background_node.background_color = BackgroundColor(Color::BLACK);
        background_node.z_index = ZIndex::Global(i32::MAX / 64);

        // Spawn the root entity
        let background = world
            .spawn((LoadingScreenRootNode, background_node, Name::new("LoadingScreenRootNode")))
            .id();

        // Build the children
        ProgressBarNode::build(world, background);
        LoadingScreenLogoNode::build(world, background);
    }

    /// Shows the loading screen.
    fn show_loadingscreen(mut query: Query<&mut Visibility, With<Self>>) {
        debug!("Showing LoadingScreen");
        for mut visibility in &mut query {
            *visibility = Visibility::Inherited;
        }
    }

    /// Hides the loading screen.
    fn hide_loadingscreen(mut query: Query<&mut Visibility, With<Self>>) {
        debug!("Hiding LoadingScreen");
        for mut visibility in &mut query {
            *visibility = Visibility::Hidden;
        }
    }

    /// Show the loading screen when resource packs are being reloaded.
    fn listen_for_resourcepacks(mut state: ResMut<NextState<LoadingScreenStateSet>>) {
        debug!("Entering LoadingScreenStateSet::Shown");
        state.set(LoadingScreenStateSet::Shown);
    }
}

/// Creates a [`NodeBundle`] that centers its children.
///
/// Fills the entire parent and has no background color.
fn center_node() -> NodeBundle {
    NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),

            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..Default::default()
        },
        background_color: BackgroundColor(Color::NONE),
        ..Default::default()
    }
}
