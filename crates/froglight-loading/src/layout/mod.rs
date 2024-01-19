//! The layout of the loading screen
use bevy::prelude::*;

pub(crate) mod loading_icon;
use loading_icon::LoadingIcon;

pub(crate) mod progress_bar;
use progress_bar::ProgressBar;

#[doc(hidden)]
pub(super) fn setup(app: &mut App) {
    loading_icon::setup(app);
    progress_bar::setup(app);
}

#[doc(hidden)]
pub(super) fn finish(app: &mut App) {
    // Create a Camera2dBundle if there isn't one already
    if app
        .world
        .query_filtered::<(), With<Camera2d>>()
        .iter(&app.world)
        .next()
        .is_none()
    {
        debug!("Creating a Camera2dBundle...");

        app.world
            .spawn(froglight_gui::default_camera::default_camera2d_bundle());
    }

    // Build the loading screen
    LoadingScreenRoot::build(app);
}

/// The a [`Component`] of the root [`Entity`] of the loading screen
///
/// All UI elements of the loading screen should be children of this [`Entity`].
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Component)]
pub(crate) struct LoadingScreenRoot;

impl LoadingScreenRoot {
    /// Build the loading screen
    ///
    /// This is not a [`System`], it is
    /// called from [`Plugin::finish`].
    fn build(app: &mut App) {
        debug!("Building loading screen...");

        // Create the root entity
        let root = app
            .world
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
        LoadingScreenCenter::build(app, root);
    }
}

/// A child [`Component`] of the [`LoadingScreenRoot`] [`Entity`] that centers
/// all UI elements of the loading screen.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Component)]
pub(crate) struct LoadingScreenCenter;

impl LoadingScreenCenter {
    /// Build the loading screen center
    fn build(app: &mut App, parent: Entity) {
        debug!("Building loading screen center...");

        let center = app
            .world
            .spawn((
                LoadingScreenCenter,
                NodeBundle {
                    style: Style {
                        width: Val::Percent(75.0),
                        height: Val::Percent(85.0),

                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
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
        LoadingIcon::build(app, center);

        // Create the progress bar
        ProgressBar::build(app, center);
    }
}
