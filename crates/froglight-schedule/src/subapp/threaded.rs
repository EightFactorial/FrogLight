#![allow(clippy::used_underscore_binding)]

use bevy_app::{AppLabel, InternedAppLabel, PluginsState, prelude::*};
use bevy_ecs::{intern::Interned, prelude::*};
#[cfg(feature = "trace")]
use bevy_log::info_span;
use bevy_tasks::{Scope, prelude::*};
use derive_more::{Deref, DerefMut};

/// A list of [`InternedAppLabel`]s to execute in parallel.
#[derive(Debug, Default, Clone, Resource, Deref, DerefMut)]
pub struct ThreadedSubApps(Vec<bevy_app::InternedAppLabel>);

impl ThreadedSubApps {
    /// Build, finish, and cleanup the app plugins.
    fn wait_for_plugins(app: &mut App) {
        let plugins_state = app.plugins_state();
        if plugins_state != PluginsState::Cleaned {
            while app.plugins_state() == PluginsState::Adding {
                bevy_tasks::tick_global_task_pools_on_main_thread();
            }
            app.finish();
            app.cleanup();
        }
    }

    /// A multi-threaded [`App`] runner.
    ///
    /// Only spawns tasks for [`SubApp`]s inside [`ThreadedSubApps`].
    pub fn threaded_runner(mut app: App) -> AppExit {
        Self::wait_for_plugins(&mut app);

        let pool = ComputeTaskPool::get();
        let mut sub_apps = Self::threaded_subapps(&mut app);

        loop {
            pool.scope::<_, ()>(|scope| {
                Self::threaded_loop(&mut app, &mut sub_apps, scope);
            });

            // Exit if requested.
            if let Some(exit) = app.should_exit() {
                return exit;
            }
        }
    }

    /// Collect all multi-threaded SubApps
    fn threaded_subapps(app: &mut App) -> Vec<(SubApp, InternedAppLabel)> {
        if let Some(labels) = app.world_mut().remove_resource::<ThreadedSubApps>() {
            labels
                .0
                .into_iter()
                .filter_map(|label| app.remove_sub_app(label).zip(Some(label)))
                .collect()
        } else {
            Vec::new()
        }
    }

    /// Spawn threads for all multi-threaded SubApps.
    #[inline]
    fn threaded_loop<'a>(
        app: &mut App,
        sub_apps: &'a mut [(SubApp, Interned<dyn AppLabel>)],
        scope: &Scope<'a, '_, ()>,
    ) {
        #[cfg(feature = "trace")]
        let _span = info_span!("threaded update").entered();

        // Sync and spawn tasks for all multi-threaded SubApps.
        for (sub_app, _label) in sub_apps {
            #[cfg(feature = "trace")]
            let _span = info_span!("threaded extract", name = ?_label).entered();

            sub_app.extract(app.world_mut());
            scope.spawn(async {
                #[cfg(feature = "trace")]
                let _span = info_span!("threaded subapp", name = ?_label.clone()).entered();

                sub_app.update();
            });
        }

        #[cfg(feature = "trace")]
        drop(_span);

        // Update the App and other SubApps while those process.
        app.update();
    }
}
