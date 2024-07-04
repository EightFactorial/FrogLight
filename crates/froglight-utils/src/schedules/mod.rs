//! [`Schedules`] that run at specific intervals.

use bevy_app::{App, MainScheduleOrder, RunFixedMainLoop};
use bevy_ecs::schedule::{ExecutorKind, Schedule, ScheduleLabel};

mod current_tick;
pub use current_tick::*;

mod seconds;
pub use seconds::{FiveSeconds, OneSecond};

mod ticks;
pub use ticks::{OneTick, TenTicks, TwoTicks};

mod timer;

mod traits;
use traits::ScheduleTrait;

pub(super) fn build(app: &mut App) {
    // Create and add the main schedule to the app
    let mut schedule = Schedule::new(RunFixedUtilLoop);
    schedule.set_executor_kind(ExecutorKind::SingleThreaded);
    app.add_schedule(schedule);

    // Insert the main schedule into the schedule order
    let mut order = app.world_mut().resource_mut::<MainScheduleOrder>();
    order.insert_after(RunFixedMainLoop, RunFixedUtilLoop);

    // Add virtual time schedules
    OneTick::build(app);
    TwoTicks::build(app);
    TenTicks::build(app);

    // Add real time schedules
    OneSecond::build(app);
    FiveSeconds::build(app);

    // Add CurrentTick
    current_tick::build(app);
}

/// Runs all fixed timer schedules in a loop according until all relevant
/// elapsed time has been "consumed".
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, ScheduleLabel)]
pub struct RunFixedUtilLoop;
