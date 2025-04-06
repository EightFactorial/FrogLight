//! [`SubAppPlugin`] for creating [`SubApp`]s.

use bevy_app::{AppLabel, MainScheduleOrder, prelude::*};
use bevy_ecs::{
    prelude::*,
    schedule::{ExecutorKind, ScheduleLabel},
};
use bevy_time::{TimeUpdateStrategy, prelude::*};
use parking_lot::Mutex;

use crate::{prelude::*, systemset::SystemSetPlugin};

mod reflect;
pub use reflect::ReflectSubAppSync;

mod sync;
pub use sync::{SubAppSync, SyncStorage};

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

            // Add `bevy_time` support.
            sub_app.init_resource::<Time>().register_type::<Time>();
            sub_app.init_resource::<Time<Real>>().register_type::<Time<Real>>();
            sub_app.init_resource::<Time<Virtual>>().register_type::<Time<Virtual>>();
            sub_app.init_resource::<Time<Fixed>>().register_type::<Time<Fixed>>();
            sub_app.init_resource::<TimeUpdateStrategy>().register_type::<Timer>();
            sub_app.add_systems(Main, bevy_time::time_system);

            // Add and initialize `CurrentTick`, `TickRate`, and `ShouldTick`.
            sub_app.init_resource::<CurrentTick>().register_type::<CurrentTick>();
            sub_app.init_resource::<TickRate>().register_type::<TickRate>();
            sub_app.init_resource::<ShouldTick>().register_type::<ShouldTick>();

            // Update the tick and run `Main` if needed.
            sub_app.add_systems(
                Main,
                (
                    ShouldTick::update_tick,
                    CurrentTick::increment_tick.run_if(ShouldTick::should_tick),
                    Main::run_main.after(bevy_time::time_system).run_if(ShouldTick::should_tick),
                )
                    .chain(),
            );

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
