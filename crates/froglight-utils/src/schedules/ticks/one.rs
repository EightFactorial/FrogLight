use bevy_ecs::schedule::ScheduleLabel;
use bevy_reflect::TypePath;
use bevy_time::Virtual;

use crate::schedules::traits::ScheduleTrait;

/// A [`Schedule`](bevy_ecs::schedule::Schedule) that runs once every tick.
///
/// Equivalent to `20` times per [`Virtual`] second.
///
/// # Example
/// ```rust,no_run
/// use bevy_app::App;
/// use froglight_utils::{schedules::OneTick, UtilityPlugin};
///
/// let mut app = App::new();
/// app.add_plugins(UtilityPlugin);
///
/// // Add a system that runs once every tick
/// app.add_systems(OneTick, || {
///     bevy_log::info!("One tick has passed!");
/// });
/// ```
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, ScheduleLabel, TypePath)]
pub struct OneTick;

impl ScheduleTrait<Virtual> for OneTick {
    /// Equivalent to `20` times per [`Virtual`] second.
    const MILLISECONDS: u64 = 1000 / 20;
}
