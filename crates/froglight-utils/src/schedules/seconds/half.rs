use bevy_ecs::schedule::ScheduleLabel;
use bevy_reflect::TypePath;

use crate::schedules::traits::ScheduleTrait;

/// A [`Schedule`](bevy_ecs::schedule::Schedule) that runs twice every second.
///
/// This schedule uses [`Real`](bevy_time::Real) time.
///
/// # Example
/// ```rust,no_run
/// use bevy_app::App;
/// use froglight_utils::{schedules::HalfSecond, UtilityPlugin};
///
/// let mut app = App::new();
/// app.add_plugins(UtilityPlugin);
///
/// // Add a system that runs twice every second
/// app.add_systems(HalfSecond, || {
///     bevy_log::info!("1/2 a second has passed!");
/// });
/// ```
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, ScheduleLabel, TypePath)]
pub struct HalfSecond;

impl ScheduleTrait for HalfSecond {
    /// Equivalent to twice every second.
    const MILLISECONDS: u64 = 1000 / 2;
}
