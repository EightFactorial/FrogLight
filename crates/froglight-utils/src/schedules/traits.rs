use std::time::Duration;

use bevy_app::App;
use bevy_ecs::{
    schedule::{ExecutorKind, Schedule, ScheduleLabel},
    world::{Mut, World},
};
use bevy_reflect::TypePath;
use bevy_time::Time;

use super::{timer::ScheduleTimer, RunFixedUtilLoop};

pub(super) trait ScheduleTrait<TimeType: 'static + Default + Send + Sync = ()>
where
    Self: Default + ScheduleLabel + TypePath,
{
    const MILLISECONDS: u64;

    fn build(app: &mut App) {
        // Create a new schedule
        let mut schedule = Schedule::new(Self::default());
        schedule.set_executor_kind(ExecutorKind::SingleThreaded);

        // Add the schedule to the app
        app.add_schedule(schedule)
            .init_resource::<ScheduleTimer<Self>>()
            .register_type::<ScheduleTimer<Self>>();

        // Add a schedule runner and timer
        app.add_systems(RunFixedUtilLoop, run_schedule::<Self, TimeType>);
    }
}

fn run_schedule<T: ScheduleTrait<TimeType>, TimeType: 'static + Default + Send + Sync>(
    world: &mut World,
) {
    world.schedule_scope(T::default(), |world, schedule| {
        world.resource_scope(|world, mut timer: Mut<ScheduleTimer<T>>| {
            // Tick the timer
            let time = world.resource::<Time<TimeType>>();
            timer.tick(time.delta());

            // Run the schedule as many times as needed
            for _ in
                0..timer.runs(Duration::from_millis(<T as ScheduleTrait<TimeType>>::MILLISECONDS))
            {
                schedule.run(world);
            }
        });
    });
}
