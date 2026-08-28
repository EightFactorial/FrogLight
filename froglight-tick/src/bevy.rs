//! TODO

use bevy_app::{MainScheduleOrder, prelude::*};
use bevy_ecs::schedule::{Schedule, SingleThreadedExecutor};

use crate::{disable::TickDisabledSet, prelude::*, schedule::RunTickLoop};

/// A [`Plugin`] that...
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TickPlugin;

impl Plugin for TickPlugin {
    fn build(&self, app: &mut App) {
        // Add the `TickCounter` resource.
        app.init_resource::<TickCounter>().register_type::<TickCounter>();

        // Add the `Ticked` event type.
        app.register_type::<Ticked>();

        // Add `TickDisabled` and `TickDisabledSet`.
        app.init_resource::<TickDisabledSet>()
            .register_type::<TickDisabledSet>()
            .register_type::<TickDisabled>();
        app.register_disabling_component::<TickDisabled>();

        // Create and add the `RunTickLoop` schedule.
        let mut tick_schedule = Schedule::new(RunTickLoop);
        tick_schedule.set_executor(SingleThreadedExecutor::new());
        app.add_schedule(tick_schedule);

        // Insert `RunTickLoop` after `RunFixedMainLoop`.
        // (Usually after `PreUpdate` and before `Update`)
        let mut schedules = app.world_mut().resource_mut::<MainScheduleOrder>();
        schedules.insert_after(RunFixedMainLoop, RunTickLoop);

        #[cfg(feature = "tracing")]
        tracing::trace!(target: "froglight_tick", "{:#?}", schedules.as_ref());

        // Add `RunTickLoop::run_tick` system to the `RunTickLoop` schedule.
        app.add_systems(RunTickLoop, RunTickLoop::run_tick);

        // Add `TickCounter::increment_counter` to the `TickFirst` schedule.
        app.add_systems(TickFirst, TickCounter::increment_counter);
        // Add `Ticked::trigger_ticked` to the `Tick` schedule.
        app.add_systems(Tick, Ticked::trigger_ticked);

        // Add the `TickMeasurementPlugin`.
        #[cfg(feature = "bevy_diagnostic")]
        app.add_plugins(crate::diagnostic::TickMeasurementPlugin);
    }
}
