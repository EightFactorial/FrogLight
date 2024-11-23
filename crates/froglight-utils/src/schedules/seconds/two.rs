use bevy_ecs::schedule::ScheduleLabel;
use bevy_reflect::TypePath;

use crate::schedules::traits::ScheduleTrait;

/// A [`Schedule`](bevy_ecs::schedule::Schedule) that runs every two seconds.
///
/// This schedule uses [`Real`](bevy_time::Real) [`Time`](bevy_time::Time).
///
/// # Example
/// ```rust,no_run
/// use bevy_app::App;
/// use froglight_utils::{schedules::TwoSeconds, UtilityPlugin};
///
/// let mut app = App::new();
/// app.add_plugins(UtilityPlugin);
///
/// // Add a system that runs every two seconds
/// app.add_systems(TwoSeconds, || {
///     bevy_log::info!("Two seconds have passed!");
/// });
/// ```
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, ScheduleLabel, TypePath)]
pub struct TwoSeconds;

impl ScheduleTrait for TwoSeconds {
    /// Equivalent to once every `2` seconds.
    const MILLISECONDS: u64 = 1000 * 2;
}
