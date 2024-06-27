use bevy_ecs::schedule::ScheduleLabel;
use bevy_reflect::TypePath;
use bevy_time::Virtual;

use crate::schedules::traits::ScheduleTrait;

/// A [`Schedule`](bevy_ecs::schedule::Schedule) that runs once every two ticks.
///
/// Equivalent to `10` times per [`Virtual`] second.
///
/// # Example
/// ```rust,no_run
/// use bevy_app::App;
/// use froglight_utils::{schedules::TwoTicks, UtilityPlugin};
///
/// let mut app = App::new();
/// app.add_plugins(UtilityPlugin);
///
/// // Add a system that runs once every two ticks
/// app.add_systems(TwoTicks, || {
///     bevy_log::info!("Two ticks have passed!");
/// });
/// ```
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, ScheduleLabel, TypePath)]
pub struct TwoTicks;

impl ScheduleTrait<Virtual> for TwoTicks {
    /// Equivalent to `10` times [`Virtual`] per second.
    const MILLISECONDS: u64 = 1000 / 10;
}
