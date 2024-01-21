//! The progress bar of the loading screen
use bevy::prelude::*;

pub(crate) mod bar_progress;
use bar_progress::ProgressBarProgress;

use crate::layout::fade_animation::FadeAnimationMarker;

#[doc(hidden)]
pub(super) fn setup(app: &mut App) { bar_progress::setup(app); }

/// The progress bar of the loading screen
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Component)]
pub(crate) struct ProgressBar;

impl ProgressBar {
    /// Create the progress bar
    pub(super) fn build_loading_bar(world: &mut World, parent: Entity) {
        let outer_node = world
            .spawn((
                FadeAnimationMarker,
                NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(25.0),

                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    ..Default::default()
                },
                #[cfg(debug_assertions)]
                Outline::new(Val::Px(1.0), Val::Auto, Color::BLUE),
            ))
            .set_parent(parent)
            .id();

        // Create the outer bar, the outer white background
        let outer_bar = world
            .spawn((
                ProgressBar,
                FadeAnimationMarker,
                NodeBundle {
                    style: Style {
                        width: Val::Percent(90.0),
                        height: Val::VMin(3.0),
                        min_height: Val::Px(25.0),

                        padding: UiRect::all(Val::Px(4.0)),
                        ..Default::default()
                    },
                    background_color: BackgroundColor(Color::WHITE),
                    ..Default::default()
                },
            ))
            .set_parent(outer_node)
            .id();

        // Create the bar background, the inner black background
        let inner_background = world
            .spawn((
                FadeAnimationMarker,
                NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),

                        padding: UiRect::all(Val::Px(4.0)),
                        ..Default::default()
                    },
                    background_color: BackgroundColor(Color::BLACK),
                    ..Default::default()
                },
            ))
            .set_parent(outer_bar)
            .id();

        ProgressBarProgress::build_loading_bar_progress(world, inner_background);
    }
}
