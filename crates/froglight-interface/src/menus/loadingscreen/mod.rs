//! The screen displayed while loading resources.
//!
//! Appears while the game is starting up or reloading resource packs.

use bevy::prelude::*;

pub(crate) mod plugin;

mod bar;
pub use bar::{ProgressBar, ProgressBarNode};

mod logo;
pub use logo::{LoadingScreenLogo, LoadingScreenLogoNode};

pub(crate) mod systemset;
use systemset::LoadingScreenPostStartupSet;

#[doc(hidden)]
fn build(app: &mut App) {
    app.register_type::<LoadingScreenRootNode>();

    app.add_systems(
        PostStartup,
        LoadingScreenRootNode::build_loadingscreen
            .run_if(not(any_with_component::<LoadingScreenRootNode>))
            .run_if(run_once())
            .in_set(LoadingScreenPostStartupSet),
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
