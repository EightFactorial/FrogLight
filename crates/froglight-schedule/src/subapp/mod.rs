//! [`SubAppPlugin`] for creating [`SubApp`]s.

use bevy_app::{AppLabel, MainScheduleOrder, prelude::*};
use bevy_ecs::{
    prelude::*,
    schedule::{ExecutorKind, ScheduleLabel},
};
use bevy_time::{Fixed, Real, Time, TimeUpdateStrategy, Virtual};
use parking_lot::Mutex;

use crate::{
    schedule::{PostNetwork, PostTick, PreNetwork, PreTick, SchedulePlugin, Tick, TickSettings},
    systemset::SystemSetPlugin,
};

mod reflect;
pub use reflect::ReflectSubAppSync;

mod sync;
pub use sync::{SubAppSync, SyncStorage};

mod tick;
pub use tick::CurrentTick;

#[cfg(test)]
mod test;

/// A [`Plugin`] that creates a new [`SubApp`] with a given [`AppLabel`].
pub struct SubAppPlugin<SubApp: AppLabel> {
    #[expect(clippy::type_complexity)]
    extract_fn: Mutex<Option<Box<dyn Fn(&mut World, &mut World) + Send>>>,
    subapp_label: Mutex<Option<SubApp>>,
}

impl<SubAppLabel: AppLabel> Plugin for SubAppPlugin<SubAppLabel> {
    fn build(&self, app: &mut App) {
        // Create a new subapp with the label.
        let mut sub_app = SubApp::new();

        {
            // Copy select resources from the main app to the subapp.
            sub_app.insert_resource(app.world().resource::<AppTypeRegistry>().clone());
        }

        {
            // Set the subapp's extract function.
            sub_app.set_extract(self.take_extract());
        }

        {
            // Add the `Main` schedule.
            let mut main_schedule = Schedule::new(Main);
            main_schedule.set_executor_kind(ExecutorKind::SingleThreaded);
            sub_app.add_schedule(main_schedule);
            sub_app.update_schedule = Some(Main.intern());

            // Add the schedules in the order they will be run.
            sub_app.insert_resource(MainScheduleOrder {
                labels: vec![
                    First.intern(),
                    PreNetwork.intern(),
                    PreTick.intern(),
                    Tick.intern(),
                    PostTick.intern(),
                    PostNetwork.intern(),
                    Last.intern(),
                ],
                ..Default::default()
            });

            // Add and register the `Time` resources
            sub_app.init_resource::<Time>().register_type::<Time>();
            sub_app.init_resource::<Time<Real>>().register_type::<Time<Real>>();
            sub_app.init_resource::<Time<Virtual>>().register_type::<Time<Virtual>>();
            sub_app.init_resource::<Time<Fixed>>().register_type::<Time<Fixed>>();
            sub_app.init_resource::<TimeUpdateStrategy>();

            // Update the time and current tick, run `Main` if needed.
            sub_app.init_resource::<TickSettings>().register_type::<TickSettings>();
            sub_app.add_systems(
                Main,
                (bevy_time::time_system, SchedulePlugin::tick_update, run_main.run_if(any_ticks))
                    .chain(),
            );

            // Add the `CurrentTick` counter and increment it every tick.
            sub_app.init_resource::<CurrentTick>().register_type::<CurrentTick>();
            sub_app.add_systems(First, CurrentTick::increment_tick);

            // Add the `SystemSetPlugin` to the subapp.
            sub_app.add_plugins(SystemSetPlugin);
        }

        // Insert the subapp into the main app.
        app.insert_sub_app(self.subapp_label.lock().take().unwrap(), sub_app);
    }

    fn finish(&self, app: &mut App) {
        // Build the subapp's `SyncStorage`.
        app.init_resource::<SyncStorage<SubAppLabel>>();
    }
}

impl<SubApp: AppLabel> SubAppPlugin<SubApp> {
    /// Creates a new [`SubAppPlugin`].
    #[inline]
    #[must_use]
    pub fn new(label: SubApp) -> Self {
        Self { extract_fn: Mutex::new(None), subapp_label: Mutex::new(Some(label)) }
    }

    /// Set the [`SubApp`]'s extract function.
    #[must_use]
    pub fn with_extract(self, extract: impl Fn(&mut World, &mut World) + Send + 'static) -> Self {
        *self.extract_fn.lock() = Some(Box::new(extract));
        self
    }

    /// Take the [`SubApp`]'s extract function.
    #[must_use]
    #[expect(clippy::type_complexity)]
    pub fn take_extract(&self) -> Box<dyn Fn(&mut World, &mut World) + Send> {
        self.extract_fn.lock().take().unwrap_or_else(|| Box::new(Self::extract))
    }

    /// The default [`SubAppPlugin`] extract function.
    ///
    /// Runs all registered [`SubAppSync`] functions from the [`SyncStorage`].
    pub fn extract(app: &mut World, sub: &mut World) {
        app.resource_scope::<SyncStorage<SubApp>, ()>(|app, storage| {
            storage.iter().for_each(|sync| sync.sync(app, sub));
        });
    }
}

/// A [`Condition`] that checks if there are any ticks to run.
fn any_ticks(ticks: Res<TickSettings>) -> bool { ticks.ticks() > 0 }

/// A [`System`] that runs [`Main::run_main`]
/// for the amount of ticks that need to be run.
fn run_main(world: &mut World) {
    let ticks = world.resource::<TickSettings>().ticks();
    (0..ticks).for_each(|_| world.run_system_cached(Main::run_main).unwrap());
}
