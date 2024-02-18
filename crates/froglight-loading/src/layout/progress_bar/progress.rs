//! The actual progress shown on the progress bar
use bevy::prelude::*;

use crate::{
    layout::fade_animation::{FadeAnimationMarker, FadeTimer},
    systemsets::{LoadingScreenFadeInSet, LoadingScreenFadeOutSet, LoadingScreenToggleSet},
};

#[doc(hidden)]
pub(super) fn setup(app: &mut App) {
    app.add_systems(
        Update,
        ProgressBarProgress::update_current_progress
            .run_if(any_with_component::<ProgressBarProgress>)
            .run_if(not(resource_exists::<FadeTimer>))
            .after(LoadingScreenFadeInSet)
            .before(LoadingScreenFadeOutSet)
            .in_set(LoadingScreenToggleSet),
    );
}

/// The actual progress shown on the progress bar
#[derive(Debug, Default, Clone, Copy, PartialEq, Component)]
pub(crate) struct ProgressBarProgress {
    pub(crate) current_progress: f32,
    pub(crate) target_progress: f32,
}

impl ProgressBarProgress {
    /// Create the progress bar progress
    pub(crate) fn build_loading_bar_progress(world: &mut World, parent: Entity) {
        world
            .spawn((
                ProgressBarProgress::default(),
                FadeAnimationMarker,
                NodeBundle {
                    style: Style {
                        width: Val::Percent(0.0),
                        height: Val::Percent(100.0),
                        ..Default::default()
                    },
                    background_color: BackgroundColor(Color::WHITE),
                    ..Default::default()
                },
            ))
            .set_parent(parent);
    }

    /// Returns `true` if the progress bar has reached the end
    pub(crate) fn bar_finished(query: Query<&ProgressBarProgress>) -> bool {
        query.iter().any(|progress| progress.current_progress >= 100.0)
    }

    /// How fast the progress bar should move towards the target progress
    const BAR_STIFFNESS: f32 = 10.0;

    /// Update the current progress towards the target progress
    ///
    /// Will not run if a fade timer exists.
    pub(crate) fn update_current_progress(
        mut query: Query<(&mut Style, &mut ProgressBarProgress)>,
        time: Res<Time<Real>>,
    ) {
        let delta = time.delta_seconds().clamp(0.0, 0.1);
        for (mut style, mut progress) in &mut query {
            let mut delta_progress = progress.target_progress - progress.current_progress;
            delta_progress *= delta * Self::BAR_STIFFNESS;

            progress.current_progress += delta_progress;
            progress.current_progress = progress.current_progress.min(100.0);

            style.width = Val::Percent(progress.current_progress);
        }
    }
}
