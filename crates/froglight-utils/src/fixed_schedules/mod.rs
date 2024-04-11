//! Various schedules that run at fixed intervals.

use std::time::Duration;

use bevy_app::{App, MainScheduleOrder, RunFixedMainLoop};
use bevy_ecs::{
    schedule::{ExecutorKind, IntoSystemConfigs, IntoSystemSet, Schedule, ScheduleLabel},
    system::Resource,
    world::{Mut, World},
};
use bevy_time::Time;

mod one_second;
pub use one_second::{OneSecondDuration, OneSecondSchedule};

mod five_seconds;
pub use five_seconds::{FiveSecondDuration, FiveSecondSchedule};

mod thirty_seconds;
pub use thirty_seconds::{ThirtySecondDuration, ThirtySecondSchedule};

/// A [`ScheduleLabel`] that runs all fixed timer schedules.
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash, ScheduleLabel)]
pub struct RunFixedTimers;

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    // Create a new schedule that runs all fixed timer schedules.
    let mut schedule = Schedule::new(RunFixedTimers);
    schedule.set_executor_kind(ExecutorKind::MultiThreaded);

    // Add the schedule to the app.
    app.add_schedule(schedule);

    // Add the schedule to the main schedule order.
    app.world.resource_mut::<MainScheduleOrder>().insert_after(RunFixedMainLoop, RunFixedTimers);

    // Add the fixed timer schedules.
    OneSecondSchedule::build(app);
    FiveSecondSchedule::build_after(app, OneSecondSchedule::fixed_timer_schedule);
    ThirtySecondSchedule::build_after(app, FiveSecondSchedule::fixed_timer_schedule);
}

/// A trait for fixed timer schedules.
trait FixedTimer: 'static + std::fmt::Debug + Default + ScheduleLabel {
    /// The duration type used to track the time since the last run.
    type ScheduleTimer: Default + Resource + std::ops::DerefMut<Target = Duration>;

    /// The number of seconds between each run.
    const SECONDS: u64;

    /// Builds the schedule.
    fn build(app: &mut App) {
        // Create a new schedule that runs every second.
        let mut schedule = Schedule::new(Self::default());
        schedule.set_executor_kind(ExecutorKind::MultiThreaded);

        // Add the schedule to the app.
        app.add_schedule(schedule)
            // Add the duration to track the time since the last run.
            .init_resource::<Self::ScheduleTimer>()
            // Add the system that runs the schedule.
            .add_systems(RunFixedTimers, Self::fixed_timer_schedule);
    }

    /// Builds the schedule and runs it after another system.
    fn build_after<M>(app: &mut App, system_set: impl IntoSystemSet<M>) {
        // Create a new schedule that runs every second.
        let mut schedule = Schedule::new(Self::default());
        schedule.set_executor_kind(ExecutorKind::MultiThreaded);

        // Add the schedule to the app.
        app.add_schedule(schedule)
            // Add the duration to track the time since the last run.
            .init_resource::<Self::ScheduleTimer>()
            // Add the system that runs the schedule.
            .add_systems(RunFixedTimers, Self::fixed_timer_schedule.after(system_set));
    }

    /// A system that runs the timer's schedule after enough time has passed.
    fn fixed_timer_schedule(world: &mut World) {
        // Use [`World::schedule_scope`] to temporarily remove the schedule.
        world.schedule_scope(Self::default(), |world, schedule| {
            // Use [`World::resource_scope`] to temporarily remove the timer.
            world.resource_scope(|world, mut timer: Mut<Self::ScheduleTimer>| {
                // Add the delta time to the duration.
                **timer += world.resource::<Time>().delta();

                // Run the schedule for each `Self::SECOND` seconds that have passed.
                let schedule_runs = timer.as_secs() / Self::SECONDS;
                // Subtract the time that will been processed.
                **timer -= Duration::from_secs(Self::SECONDS * schedule_runs);

                // Run the schedule.
                for _ in 0..schedule_runs {
                    schedule.run(world);
                }
            });
        });
    }
}
