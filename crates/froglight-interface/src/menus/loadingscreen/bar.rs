use bevy::prelude::*;
use froglight_core::events::ResourcePackFinishedLoadingEvent;

use super::systemset::LoadingScreenStateSet;

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    app.register_type::<ProgressBarNode>().register_type::<ProgressBar>();

    // React to the ResourcePackFinishedLoadingEvent event
    app.add_systems(
        Update,
        ProgressBar::listen_for_finish
            .ambiguous_with_all()
            .run_if(on_event::<ResourcePackFinishedLoadingEvent>())
            .run_if(any_with_component::<ProgressBar>)
            .in_set(LoadingScreenStateSet::Shown),
    );
    // Update the visual progress of the progress bar
    app.add_systems(
        Update,
        ProgressBar::update_visual_progress
            .ambiguous_with_all()
            .run_if(any_with_component::<ProgressBar>)
            .in_set(LoadingScreenStateSet::Shown),
    );

    // Hide the progress bar when it is finished
    //
    // TODO: Find a good way to transition to the main menu
    // instead of just hiding the loading screen
    app.add_systems(
        Update,
        ProgressBar::hide_when_finished
            .run_if(ProgressBar::is_finished)
            .run_if(any_with_component::<ProgressBar>)
            .in_set(LoadingScreenStateSet::Shown),
    );
    app.add_systems(
        OnExit(LoadingScreenStateSet::Shown),
        ProgressBar::reset_progress
            .run_if(any_with_component::<ProgressBar>)
            .in_set(LoadingScreenStateSet::Shown),
    );
}

/// A marker [`Component`] for the progress bar node.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect, Component)]
#[reflect(Component)]
pub struct ProgressBarNode;

impl ProgressBarNode {
    pub(super) fn build(world: &mut World, parent: Entity) {
        // Create the progress bar node
        let bar_node = NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,

                height: Val::Px(16.0),
                width: Val::Percent(80.0),

                bottom: Val::Percent(10.0),

                border: UiRect::all(Val::Px(1.0)),
                padding: UiRect::all(Val::Px(2.0)),

                align_items: AlignItems::Center,
                ..Default::default()
            },
            background_color: BackgroundColor(Color::NONE),
            border_color: BorderColor(Color::WHITE),
            ..Default::default()
        };

        // Spawn the progress bar node
        let node =
            world.spawn((Self, bar_node, Name::new("ProgressBarNode"))).set_parent(parent).id();

        // Create the progress bar
        ProgressBar::build(world, node);
    }
}

/// A [`Component`] for a progress bar.
///
/// This component is used to track the progress of a loading screen.
///
/// The current progress is the actual progress of the bar, while the
/// visual progress what the bar looks like on the screen.
#[derive(Debug, Default, Clone, Copy, PartialEq, Reflect, Component)]
#[reflect(Component)]
pub struct ProgressBar {
    /// The current progress of the bar.
    pub current_progress: f32,
    /// The visual progress of the bar.
    ///
    /// This is used to animate the progress bar.
    pub visual_progress: f32,
}

impl ProgressBar {
    pub(super) fn build(world: &mut World, parent: Entity) {
        // Create the progress bar
        let progress = Self::default();
        let bar = NodeBundle {
            style: Style {
                height: Val::Percent(100.0),
                width: Val::Percent(progress.visual_progress),
                ..Default::default()
            },
            background_color: BackgroundColor(Color::WHITE),
            ..Default::default()
        };

        // Spawn the progress bar
        world.spawn((progress, bar, Name::new("ProgressBar"))).set_parent(parent);
    }

    /// The strength of the interpolation.
    const STRENGTH: f32 = 5.0;

    /// Smoothly interpolate the visual progress to the current progress.
    fn update_visual_progress(mut query: Query<(&mut Style, &mut Self)>, time: Res<Time<Virtual>>) {
        // Calculate the delta time
        let delta = time.delta_seconds().min(0.1) * Self::STRENGTH;

        // Update the visual progress
        for (mut style, mut progress) in &mut query {
            // Skip if the visual progress is already at the current progress
            if (progress.visual_progress - progress.current_progress).abs() < f32::EPSILON {
                continue;
            }

            // Interpolate the visual progress
            let diff = (progress.current_progress - progress.visual_progress).max(0.1);
            progress.visual_progress = (progress.visual_progress + (diff * delta)).min(100.0);
            style.width = Val::Percent(progress.visual_progress);
        }
    }

    /// Set the current progress to 100% when
    /// resource packs are finished loading.
    fn listen_for_finish(mut query: Query<&mut Self>) {
        for mut progress in &mut query {
            progress.current_progress = 100.0;
        }
    }

    /// Returns `true` if the progress bar is visually finished.
    fn is_finished(progress: Query<&Self>) -> bool {
        progress.iter().any(|p| (p.visual_progress - 100.0).abs() < f32::EPSILON)
    }

    /// Enter the hidden state when the progress bar is finished.
    fn hide_when_finished(mut state: ResMut<NextState<LoadingScreenStateSet>>) {
        debug!("Entering LoadingScreenStateSet::Hidden");
        state.set(LoadingScreenStateSet::Hidden);
    }

    /// Reset the progress of the progress bar.
    fn reset_progress(mut query: Query<&mut Self>) {
        for mut progress in &mut query {
            progress.current_progress = 0.0;
            progress.visual_progress = 0.0;
        }
    }
}
