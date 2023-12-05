use bevy::{prelude::*, ui::FocusPolicy};

use crate::menus::traits::MenuComponent;

use super::{resources::MenuResources, states::assets::AssetLoadingState};

pub mod background;
pub mod logo;
pub mod progress;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Component)]
pub struct AppLoadingNodeComponent;

impl AppLoadingNodeComponent {
    pub(super) fn setup(app: &mut App) {
        app.init_resource::<LoadingFadeTimer>();
        app.add_systems(Startup, Self::build);

        app.add_systems(
            Update,
            LoadingFadeTimer::fade_in.run_if(
                not(in_state(AssetLoadingState::Finished))
                    .and_then(resource_exists::<LoadingFadeTimer>())
                    .and_then(LoadingFadeTimer::is_fade_in),
            ),
        );

        app.add_systems(
            Update,
            LoadingFadeTimer::fade_out.run_if(
                in_state(AssetLoadingState::Finished).and_then(
                    resource_exists::<LoadingFadeTimer>()
                        .and_then(LoadingFadeTimer::is_fade_out)
                        .and_then(MenuResources::loaded),
                ),
            ),
        );

        background::BackgroundNodeComponent::setup(app);
        progress::ProgressNodeComponent::setup(app);
        logo::LogoNodeComponent::setup(app);
    }

    fn build(world: &mut World) {
        #[cfg(any(debug_assertions, feature = "debug"))]
        debug!("Building AppLoadingNodeComponent");

        let entity = world
            .spawn((
                AppLoadingNodeComponent,
                FadeComponent,
                NodeBundle {
                    style: Style {
                        position_type: PositionType::Absolute,
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        ..Default::default()
                    },
                    visibility: Visibility::Visible,
                    z_index: ZIndex::Global(i32::MAX - 32),
                    focus_policy: FocusPolicy::Block,
                    ..Default::default()
                },
            ))
            .id();

        background::BackgroundNodeComponent::build(entity, world);
        progress::ProgressNodeComponent::build(entity, world);
        logo::LogoNodeComponent::build(entity, world);
    }
}

/// A marker component for entities fading in and out with the loading screen
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Component)]
pub struct FadeComponent;

/// A timer resource for keeping track of the fade
#[derive(Debug, PartialEq, Eq, Resource)]
struct LoadingFadeTimer {
    timer: Timer,
    mode: FadeTimerMode,
}

/// The mode of the fade timer
///
/// This is used to determine if the timer is fading in or out
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FadeTimerMode {
    FadeIn,
    FadeOut,
}

impl LoadingFadeTimer {
    /// Returns `true` if the timer is in the fade in mode
    fn is_fade_in(timer: Res<LoadingFadeTimer>) -> bool { timer.mode == FadeTimerMode::FadeIn }

    /// Fades in the loading screen
    fn fade_in(
        mut query: Query<(&mut BackgroundColor, &mut Visibility), With<FadeComponent>>,
        mut timer: ResMut<LoadingFadeTimer>,
        time: Res<Time<Real>>,
        mut commands: Commands,
    ) {
        // Tick the timer
        timer.timer.tick(time.delta());

        #[cfg(any(debug_assertions, feature = "debug"))]
        trace!("FadeIn: {}", timer.timer.percent());

        for (mut color, mut vis) in query.iter_mut() {
            // Only set the visibility if it's explicitly set to hidden
            if matches!(*vis, Visibility::Hidden) {
                *vis = Visibility::Visible;
            }

            // Set the alpha to the percent completed
            color.0.set_a(timer.timer.percent());

            // Delete the timer
            if timer.timer.finished() {
                commands.remove_resource::<LoadingFadeTimer>();
            }
        }
    }

    /// Returns `true` if the timer is in the fade out mode
    fn is_fade_out(timer: Res<LoadingFadeTimer>) -> bool { timer.mode == FadeTimerMode::FadeOut }

    /// Fades out the loading screen
    fn fade_out(
        mut query: Query<(&mut BackgroundColor, &mut Visibility), With<FadeComponent>>,
        mut timer: ResMut<LoadingFadeTimer>,
        time: Res<Time<Real>>,
        mut commands: Commands,
    ) {
        // Tick the timer
        timer.timer.tick(time.delta());

        #[cfg(any(debug_assertions, feature = "debug"))]
        trace!("FadeOut: {}", timer.timer.percent_left());

        for (mut color, mut vis) in query.iter_mut() {
            // Set the alpha to the percent left
            color.0.set_a(timer.timer.percent_left());

            if timer.timer.finished() {
                // Only set the visibility if it's explicitly set to visible
                if matches!(*vis, Visibility::Visible) {
                    *vis = Visibility::Hidden;
                }

                color.0.set_a(1.0);
                commands.remove_resource::<LoadingFadeTimer>();
            }
        }
    }
}

impl Default for LoadingFadeTimer {
    fn default() -> Self { Self::from(FadeTimerMode::FadeOut) }
}

impl From<FadeTimerMode> for LoadingFadeTimer {
    fn from(value: FadeTimerMode) -> Self {
        match value {
            FadeTimerMode::FadeIn => Self {
                timer: Timer::from_seconds(0.5, TimerMode::Once),
                mode: FadeTimerMode::FadeIn,
            },
            FadeTimerMode::FadeOut => Self {
                timer: Timer::from_seconds(0.5, TimerMode::Once),
                mode: FadeTimerMode::FadeOut,
            },
        }
    }
}
