use bevy_ecs::schedule::ScheduleLabel;
use bevy_reflect::TypePath;
use bevy_time::Virtual;

use crate::schedules::traits::ScheduleTrait;

/// A [`Schedule`](bevy_ecs::schedule::Schedule) that runs once every ten ticks.
///
/// Equivalent to 2 times per [`Virtual`] second.
///
/// # Example
/// ```rust,no_run
/// use bevy_app::App;
/// use froglight_utils::{schedules::TenTicks, UtilityPlugin};
///
/// let mut app = App::new();
/// app.add_plugins(UtilityPlugin);
///
/// // Add a system that runs once every ten ticks
/// app.add_systems(TenTicks, || {
///     bevy_log::info!("Ten ticks have passed!");
/// });
/// ```
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, ScheduleLabel, TypePath)]
pub struct TenTicks;

impl ScheduleTrait<Virtual> for TenTicks {
    /// Equivalent to 2 times per [`Virtual`] second.
    const MILLISECONDS: u64 = 1000 / 2;
}
