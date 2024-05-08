//! [`Schedules`] that run at specific intervals.

use bevy_app::{App, MainScheduleOrder, RunFixedMainLoop};
use bevy_ecs::schedule::{ExecutorKind, Schedule, ScheduleLabel};

mod seconds;
pub use seconds::*;

mod ticks;
pub use ticks::*;

mod timer;

mod traits;
use traits::ScheduleTrait;

pub(super) fn build(app: &mut App) {
    // Create and add the main schedule to the app
    {
        let mut schedule = Schedule::new(RunFixedUtilLoop);
        schedule.set_executor_kind(ExecutorKind::SingleThreaded);
        app.add_schedule(schedule);
    }

    // Insert the main schedule into the schedule order
    {
        let mut order = app.world.resource_mut::<MainScheduleOrder>();
        order.insert_after(RunFixedMainLoop, RunFixedUtilLoop);
    }

    // Add all of the other schedules
    {
        OneTick::build(app);
        TwoTicks::build(app);
        TenTicks::build(app);

        OneSecond::build(app);
        FiveSeconds::build(app);
    }
}

/// Runs all fixed timer schedules in a loop according until all relevant
/// elapsed time has been "consumed".
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, ScheduleLabel)]
pub struct RunFixedUtilLoop;
