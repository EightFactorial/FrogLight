//! The layout of the loading screen
use std::time::Duration;

use bevy::prelude::*;
use froglight_core::{
    resources::loading::LoadingScreenEnable, systemsets::loading::LoadingScreenUpdateSet,
};

use super::{progress_bar::bar_progress::ProgressBarProgress, LoadingScreenRoot};
use crate::systemsets::LoadingScreenFadeOutUpdateSet;

#[doc(hidden)]
pub(super) fn setup(app: &mut App) {
    // Add fade-in systems
    app.add_systems(
        Update,
        (
            FadeTimer::insert_fade_in_timer
                .run_if(resource_exists_and_changed::<LoadingScreenEnable>())
                .run_if(not(resource_added::<LoadingScreenEnable>()))
                .run_if(not(resource_exists::<FadeTimer>())),
            FadeAnimationMarker::fade_in.run_if(resource_exists::<FadeTimer>()),
        )
            .chain()
            .in_set(LoadingScreenUpdateSet),
    );

    // Add fade-out systems
    app.add_systems(
        Update,
        (
            FadeTimer::insert_fade_out_timer
                .run_if(ProgressBarProgress::bar_finished)
                .run_if(not(resource_exists::<FadeTimer>())),
            FadeAnimationMarker::fade_out.run_if(resource_exists::<FadeTimer>()),
        )
            .chain()
            .in_set(LoadingScreenFadeOutUpdateSet),
    );
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Component)]
pub(crate) struct FadeAnimationMarker;

impl FadeAnimationMarker {
    fn fade_in(
        mut query: Query<&mut BackgroundColor, With<Self>>,
        mut root_query: Query<&mut Visibility, (With<Self>, With<LoadingScreenRoot>)>,

        mut commands: Commands,
        mut timer: ResMut<FadeTimer>,
        time: Res<Time<Real>>,
    ) {
        if timer.finished() {
            return;
        } else if timer.percent() < 0.1 {
            // Show the loading screen
            for mut visibility in &mut root_query {
                *visibility = Visibility::Visible;
            }
        }

        let delta = time.delta_seconds().clamp(0.0, 0.1);
        timer.tick(Duration::from_secs_f32(delta));

        if timer.just_finished() {
            // Reset all background colors
            for mut background_color in &mut query {
                background_color.0.set_a(1.0);
            }

            // Delete the timer
            commands.remove_resource::<FadeTimer>();
            debug!("Fade-in timer deleted");
        } else {
            // Get the opacity from the timer progress
            let progress = timer.percent();

            // Set the opacity of all background colors
            for mut background_color in &mut query {
                background_color.0.set_a(progress);
            }
        }
    }

    #[allow(clippy::type_complexity)]
    fn fade_out(
        mut query: Query<&mut BackgroundColor, With<Self>>,
        mut root_query: Query<&mut Visibility, (With<Self>, With<LoadingScreenRoot>)>,

        mut commands: Commands,
        mut timer: ResMut<FadeTimer>,
        time: Res<Time<Real>>,
    ) {
        if timer.finished() {
            return;
        }

        let delta = time.delta_seconds().clamp(0.0, 0.1);
        timer.tick(Duration::from_secs_f32(delta));

        if timer.just_finished() {
            // Hide the loading screen
            for mut visibility in &mut root_query {
                *visibility = Visibility::Hidden;
            }

            // Reset all background colors
            for mut background_color in &mut query {
                background_color.0.set_a(1.0);
            }

            // Delete the timer
            commands.remove_resource::<FadeTimer>();
            debug!("Fade-out timer deleted");
        } else {
            // Get the opacity from the timer progress
            let progress = timer.percent_left();

            // Set the opacity of all background colors
            for mut background_color in &mut query {
                background_color.0.set_a(progress);
            }
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Deref, DerefMut, Resource)]
struct FadeTimer(Timer);

impl FadeTimer {
    const FADE_IN_DURATION: f32 = 0.5;
    const FADE_OUT_DURATION: f32 = 0.5;

    /// Insert a fade in timer
    fn insert_fade_in_timer(mut commands: Commands) {
        debug!("Inserting fade-in timer...");

        commands
            .insert_resource(Self(Timer::from_seconds(Self::FADE_IN_DURATION, TimerMode::Once)));
    }

    /// Insert a fade out timer
    fn insert_fade_out_timer(mut commands: Commands) {
        debug!("Inserting fade-out timer...");

        commands
            .insert_resource(Self(Timer::from_seconds(Self::FADE_OUT_DURATION, TimerMode::Once)));
    }
}
