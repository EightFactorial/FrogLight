use bevy_ecs::schedule::ScheduleLabel;
use bevy_reflect::TypePath;

use crate::schedules::traits::ScheduleTrait;

/// A [`Schedule`](bevy_ecs::schedule::Schedule) that runs every five seconds.
///
/// This schedule uses [`Real`](bevy_time::Real) [`Time`](bevy_time::Time).
///
/// # Example
/// ```rust,no_run
/// use bevy_app::App;
/// use froglight_utils::{schedules::FiveSeconds, UtilityPlugin};
///
/// let mut app = App::new();
/// app.add_plugins(UtilityPlugin);
///
/// // Add a system that runs every five seconds
/// app.add_systems(FiveSeconds, || {
///     bevy_log::info!("Five seconds have passed!");
/// });
/// ```
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, ScheduleLabel, TypePath)]
pub struct FiveSeconds;

impl ScheduleTrait for FiveSeconds {
    /// Equivalent to every `5` seconds.
    const MILLISECONDS: u64 = 1000 * 5;
}
