//! [`SubAppPlugin`] for creating [`SubApp`]s.

use std::time::Instant;

use bevy_app::{AppLabel, MainScheduleOrder, prelude::*};
use bevy_ecs::{
    prelude::*,
    schedule::{ExecutorKind, ScheduleLabel},
};
use bevy_time::{TimeUpdateStrategy, prelude::*};
use parking_lot::Mutex;

use crate::{prelude::*, schedule::SystemSetPlugin};

mod reflect;
pub use reflect::{AppSyncStorage, ReflectSubAppSync, SubAppSync};

#[cfg(feature = "multi_threaded")]
mod threaded;
#[cfg(feature = "multi_threaded")]
pub use threaded::ThreadedSubApps;

/// A [`Plugin`] that creates a new [`SubApp`] with a given [`AppLabel`].
pub struct SubAppPlugin<SubApp: AppLabel> {
    #[expect(clippy::type_complexity)]
    extract_fn: Mutex<Option<Box<dyn Fn(&mut World, &mut World) + Send>>>,
    subapp_label: Mutex<Option<SubApp>>,
}

impl<SubAppLabel: AppLabel> Plugin for SubAppPlugin<SubAppLabel> {
    fn build(&self, app: &mut App) {
        assert!(
            app.get_sub_app(self.subapp_label.lock().as_ref().unwrap().intern()).is_none(),
            "`SubAppPlugin<{}>` tried to create a `SubApp` that already exists!",
            std::any::type_name::<SubAppLabel>()
        );

        app.register_type::<SubWorlds>();

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
            // Add the `Main` schedule and set it as the update schedule.
            let mut main_schedule = Schedule::new(Main);
            main_schedule.set_executor_kind(ExecutorKind::SingleThreaded);
            sub_app.add_schedule(main_schedule).update_schedule = Some(Main.intern());

            // Add the schedules in the order they will be run.
            sub_app.insert_resource(MainScheduleOrder {
                labels: vec![
                    First.intern(),
                    Network::PreNetwork.intern(),
                    Tick::PreTick.intern(),
                    Tick::Tick.intern(),
                    Tick::PostTick.intern(),
                    Network::PostNetwork.intern(),
                    Last.intern(),
                ],
                ..Default::default()
            });

            // Add `bevy_time` support.
            sub_app.init_resource::<Time>().register_type::<Time>();
            sub_app.init_resource::<Time<Real>>().register_type::<Time<Real>>();
            sub_app.init_resource::<Time<Virtual>>().register_type::<Time<Virtual>>();
            sub_app.init_resource::<Time<Fixed>>().register_type::<Time<Fixed>>();
            sub_app.init_resource::<TimeUpdateStrategy>().register_type::<Timer>();

            // Add and initialize `CurrentTick`, `TickRate`, and `ShouldTick`.
            sub_app.init_resource::<CurrentTick>().register_type::<CurrentTick>();
            sub_app.init_resource::<TickRate>().register_type::<TickRate>();
            sub_app.init_resource::<ShouldTick>().register_type::<ShouldTick>();

            // Update the time and tick, run `Main` if needed.
            sub_app.add_systems(
                Main,
                (
                    time_system,
                    ShouldTick::update,
                    CurrentTick::increment_tick.run_if(ShouldTick::tick),
                    Main::run_main.run_if(ShouldTick::tick),
                )
                    .chain(),
            );

            // Add the `SystemSetPlugin` to the subapp.
            sub_app.add_plugins(SystemSetPlugin);
        }

        #[cfg(feature = "multi_threaded")]
        {
            // Setup the multi-threaded executor if not already set.
            if !app.world().contains_resource::<ThreadedSubApps>() {
                app.set_runner(ThreadedSubApps::threaded_runner);
            }
            // Add the label to the list of multi-threaded subapps.
            app.world_mut()
                .get_resource_or_init::<ThreadedSubApps>()
                .push(self.subapp_label.lock().as_ref().unwrap().intern());
        }

        // Insert the subapp into the main app.
        app.insert_sub_app(self.subapp_label.lock().take().unwrap(), sub_app);
    }

    fn finish(&self, app: &mut App) {
        // Initialize a `SyncStorage` if it does not already exist.
        app.init_resource::<AppSyncStorage<SubAppLabel>>();
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
        self.extract_fn.lock().take().unwrap_or_else(|| Box::new(Self::default_extract))
    }

    /// The default [`SubAppPlugin`] extract function.
    ///
    /// Runs all registered [`SubAppSync`] functions from the [`SyncStorage`].
    pub fn default_extract(app: &mut World, sub: &mut World) {
        app.resource_scope::<AppSyncStorage<SubApp>, ()>(|app, storage| {
            storage.iter().for_each(|sync| sync.sync(app, sub));
        });
    }
}

// -------------------------------------------------------------------------------------------------

/// The system used to update the [`Time`] used by app logic.
pub fn time_system(
    mut generic_time: ResMut<Time>,
    mut real_time: ResMut<Time<Real>>,
    mut virtual_time: ResMut<Time<Virtual>>,
    update_strategy: Res<TimeUpdateStrategy>,
) {
    match update_strategy.as_ref() {
        TimeUpdateStrategy::Automatic => real_time.update_with_instant(Instant::now()),
        TimeUpdateStrategy::ManualInstant(instant) => real_time.update_with_instant(*instant),
        TimeUpdateStrategy::ManualDuration(duration) => real_time.update_with_duration(*duration),
    }

    bevy_time::update_virtual_time(&mut generic_time, &mut virtual_time, &real_time);
}
