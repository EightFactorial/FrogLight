use bevy_ecs::schedule::ScheduleLabel;
use bevy_reflect::TypePath;

use crate::schedules::traits::ScheduleTrait;

/// A [`Schedule`](bevy_ecs::schedule::Schedule) that runs once every second.
///
/// This schedule uses [`Real`](bevy_time::Real) time.
///
/// # Example
/// ```rust,no_run
/// use bevy_app::App;
/// use froglight_utils::{schedules::OneSecond, UtilityPlugin};
///
/// let mut app = App::new();
/// app.add_plugins(UtilityPlugin);
///
/// // Add a system that runs once every second
/// app.add_systems(OneSecond, || {
///     bevy_log::info!("One second has passed!");
/// });
/// ```
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, ScheduleLabel, TypePath)]
pub struct OneSecond;

impl ScheduleTrait for OneSecond {
    /// Equivalent to once every second.
    const MILLISECONDS: u64 = 1000;
}
