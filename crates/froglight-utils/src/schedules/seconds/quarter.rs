use bevy_ecs::schedule::ScheduleLabel;
use bevy_reflect::TypePath;

use crate::schedules::traits::ScheduleTrait;

/// A [`Schedule`](bevy_ecs::schedule::Schedule) that runs four times every
/// second.
///
/// This schedule uses [`Real`](bevy_time::Real) [`Time`](bevy_time::Time).
///
/// # Example
/// ```rust,no_run
/// use bevy_app::App;
/// use froglight_utils::{schedules::QuarterSecond, UtilityPlugin};
///
/// let mut app = App::new();
/// app.add_plugins(UtilityPlugin);
///
/// // Add a system that runs four times every second
/// app.add_systems(QuarterSecond, || {
///     bevy_log::info!("1/4 a second has passed!");
/// });
/// ```
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, ScheduleLabel, TypePath)]
pub struct QuarterSecond;

impl ScheduleTrait for QuarterSecond {
    /// Equivalent to four times every second.
    const MILLISECONDS: u64 = 1000 / 4;
}
