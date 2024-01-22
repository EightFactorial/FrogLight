//! The layout of the loading screen
use std::time::Duration;

use bevy::prelude::*;

use super::{progress_bar::progress::ProgressBarProgress, LoadingScreenRoot};
use crate::systemsets::{
    LoadingScreenEnableSystems, LoadingScreenFadeInSet, LoadingScreenFadeOutSet,
};

#[doc(hidden)]
pub(super) fn setup(app: &mut App) {
    // Add fade-in systems
    app.add_systems(
        Update,
        (
            FadeTimer::insert_fade_in_timer
                .run_if(resource_exists_and_changed::<LoadingScreenEnableSystems>())
                .run_if(resource_exists_and_equals(LoadingScreenEnableSystems(true)))
                .run_if(not(resource_added::<LoadingScreenEnableSystems>()))
                .run_if(not(resource_exists::<FadeTimer>())),
            FadeAnimationMarker::fade_in
                .run_if(resource_exists::<FadeTimer>().and_then(FadeTimer::is_fade_in)),
        )
            .chain()
            .in_set(LoadingScreenFadeInSet),
    );

    // Add fade-out systems
    app.add_systems(
        Update,
        (
            FadeTimer::insert_fade_out_timer
                .run_if(LoadingScreenRoot::is_visible)
                .run_if(ProgressBarProgress::bar_finished)
                .run_if(not(resource_exists::<FadeTimer>())),
            FadeAnimationMarker::fade_out
                .run_if(resource_exists::<FadeTimer>().and_then(FadeTimer::is_fade_out)),
        )
            .chain()
            .in_set(LoadingScreenFadeOutSet),
    );
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Component)]
pub(crate) struct FadeAnimationMarker;

impl FadeAnimationMarker {
    pub(crate) fn fade_in(
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
            for mut color in &mut query {
                color.0.set_a(1.0);
            }

            // Delete the timer
            commands.remove_resource::<FadeTimer>();
            debug!("Fade-in timer deleted");
        } else {
            // Get the opacity from the timer progress
            let progress = timer.percent();

            // Set the opacity of all background colors
            for mut color in &mut query {
                color.0.set_a(progress);
            }
        }
    }

    pub(crate) fn fade_out(
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
            for mut color in &mut query {
                color.0.set_a(1.0);
            }

            // Delete the timer
            commands.remove_resource::<FadeTimer>();
            debug!("Fade-out timer deleted");
        } else {
            // Get the opacity from the timer progress
            let progress = timer.percent_left();

            // Set the opacity of all background colors
            for mut color in &mut query {
                color.0.set_a(progress);
            }
        }
    }
}

/// A timer that fades in or out
#[derive(Debug, Default, Clone, PartialEq, Eq, Deref, DerefMut, Resource)]
pub(crate) struct FadeTimer {
    /// The timer
    #[deref]
    pub(crate) timer: Timer,
    /// Direction of the timer
    ///
    /// `true` if the timer is fading out, `false` if the timer is fading in
    pub(crate) dir: bool,
}

impl FadeTimer {
    const FADE_IN_DURATION: f32 = 0.5;
    const FADE_OUT_DURATION: f32 = 0.5;

    /// Returns `true` if the timer is fading in
    pub(crate) fn is_fade_in(timer: Res<Self>) -> bool { !timer.dir }

    /// Create a new fade in timer
    pub(crate) fn new_fade_in() -> Self {
        Self { timer: Timer::from_seconds(Self::FADE_IN_DURATION, TimerMode::Once), dir: false }
    }

    /// Insert a fade in timer
    fn insert_fade_in_timer(mut commands: Commands) {
        debug!("Inserting fade-in timer...");
        commands.insert_resource(Self::new_fade_in());
    }

    /// Returns `true` if the timer is fading out
    pub(crate) fn is_fade_out(timer: Res<Self>) -> bool { timer.dir }

    /// Create a new fade out timer
    pub(crate) fn new_fade_out() -> Self {
        Self { timer: Timer::from_seconds(Self::FADE_OUT_DURATION, TimerMode::Once), dir: true }
    }

    /// Insert a fade out timer
    fn insert_fade_out_timer(mut commands: Commands) {
        debug!("Inserting fade-out timer...");
        commands.insert_resource(Self::new_fade_out());
    }
}
