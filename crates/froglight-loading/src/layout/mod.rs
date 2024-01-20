//! The layout of the loading screen
use bevy::prelude::*;

pub(crate) mod loading_icon;
use loading_icon::LoadingIcon;

pub(crate) mod progress_bar;
use progress_bar::ProgressBar;

use crate::systemsets::LoadingScreenStartupSet;

#[doc(hidden)]
pub(super) fn setup(app: &mut App) {
    loading_icon::setup(app);
    progress_bar::setup(app);

    app.add_systems(
        Startup,
        (
            LoadingScreenRoot::build_loading_screen
                .run_if(not(any_with_component::<LoadingScreenRoot>())),
            LoadingScreenRoot::create_camera2d_if_none
                .run_if(not(any_with_component::<Camera2d>())),
        )
            .chain()
            .in_set(LoadingScreenStartupSet),
    );
}

/// The a [`Component`] of the root [`Entity`] of the loading screen
///
/// All UI elements of the loading screen should be children of this [`Entity`].
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Component)]
pub(crate) struct LoadingScreenRoot;

impl LoadingScreenRoot {
    /// Create a Camera2d if one does not exist
    fn create_camera2d_if_none(mut commands: Commands) {
        debug!("Creating a Camera2dBundle...");
        commands.spawn(froglight_gui::default_camera::default_camera2d_bundle());
    }

    /// Build the loading screen
    ///
    /// This is not a [`System`], it is
    /// called from [`Plugin::finish`].
    fn build_loading_screen(world: &mut World) {
        debug!("Building loading screen...");

        // Create the root entity
        let root = world
            .spawn((
                LoadingScreenRoot,
                NodeBundle {
                    style: Style {
                        position_type: PositionType::Absolute,

                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),

                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    background_color: BackgroundColor(Color::BLACK),
                    z_index: ZIndex::Global(i32::MAX - 64),
                    visibility: Visibility::Visible,
                    ..Default::default()
                },
            ))
            .id();

        // Create the center entity
        LoadingScreenCenter::build_loading_center(world, root);
    }
}

/// A child [`Component`] of the [`LoadingScreenRoot`] [`Entity`] that centers
/// all UI elements of the loading screen.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Component)]
pub(crate) struct LoadingScreenCenter;

impl LoadingScreenCenter {
    /// Build the loading screen center node
    fn build_loading_center(world: &mut World, parent: Entity) {
        let center = world
            .spawn((
                LoadingScreenCenter,
                NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        width: Val::Percent(75.0),
                        height: Val::Percent(85.0),
                        ..Default::default()
                    },
                    #[cfg(debug_assertions)]
                    background_color: BackgroundColor(Color::MIDNIGHT_BLUE),
                    ..Default::default()
                },
            ))
            .set_parent(parent)
            .id();

        // Create the loading icon
        LoadingIcon::build_loading_icon(world, center);

        // Create the progress bar
        ProgressBar::build_loading_bar(world, center);
    }
}
