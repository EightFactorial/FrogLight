use std::time::Duration;

use bevy_app::{App, Update};
use bevy_derive::{Deref, DerefMut};
use bevy_ecs::{
    reflect::ReflectResource,
    schedule::IntoSystemConfigs,
    system::{Res, ResMut, Resource},
};
use bevy_reflect::{prelude::ReflectDefault, Reflect};
use bevy_state::state::{NextState, OnEnter};
use bevy_time::{Real, Time, Timer, TimerMode};

use super::AssetLoadState;

#[doc(hidden)]
pub(crate) fn build(app: &mut App) {
    app.register_type::<SpawnTimer>().init_resource::<SpawnTimer>();

    // Reset the SpawnTimer when entering the `AssetLoadState::Spawning` state
    app.add_systems(OnEnter(AssetLoadState::Spawning), SpawnTimer::reset_timer);

    // Wait for the SpawnTimer to finish,
    // then enter the `AssetLoadState::Finished` state
    app.add_systems(
        Update,
        SpawnTimer::wait_for_spawns.run_if(SpawnTimer::is_running).in_set(AssetLoadState::Spawning),
    );
}

#[derive(Debug, PartialEq, Eq, Deref, DerefMut, Reflect, Resource)]
#[reflect(Default, Resource)]
struct SpawnTimer(Timer);

impl Default for SpawnTimer {
    fn default() -> Self { Self(Timer::new(Self::TIMER_DURATION, TimerMode::Once)) }
}

impl SpawnTimer {
    const TIMER_DURATION: Duration = Duration::from_millis(1000 / 8);

    /// Returns `true` if the [`SpawnTimer`] has not finished.
    fn is_running(res: Res<Self>) -> bool { !res.finished() }

    /// When the [`SpawnTimer`] finishes,
    /// enter the [`AssetLoadState::Finished`] state.
    fn wait_for_spawns(
        time: Res<Time<Real>>,
        mut res: ResMut<Self>,
        mut state: ResMut<NextState<AssetLoadState>>,
    ) {
        if res.tick(time.delta()).just_finished() {
            #[cfg(debug_assertions)]
            bevy_log::info!("AssetLoadState: Entering `AssetLoadState::Finished`");
            state.set(AssetLoadState::Finished);
        };
    }

    /// Resets the [`SpawnTimer`].
    fn reset_timer(mut res: ResMut<Self>) { res.reset(); }
}
