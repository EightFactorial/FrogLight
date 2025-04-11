#![allow(clippy::used_underscore_binding)]

use bevy_app::{InternedAppLabel, PluginsState, prelude::*};
use bevy_ecs::prelude::*;
#[cfg(feature = "trace")]
use bevy_log::info_span;
use bevy_tasks::prelude::*;
use derive_more::{Deref, DerefMut};

/// A list of [`InternedAppLabel`]s to execute in parallel.
#[derive(Debug, Default, Clone, Resource, Deref, DerefMut)]
pub struct ThreadedSubApps(Vec<bevy_app::InternedAppLabel>);

impl ThreadedSubApps {
    /// A multi-threaded [`App`] runner.
    ///
    /// Only spawns tasks for [`SubApp`]s inside [`ThreadedSubApps`].
    pub fn threaded_runner(mut app: App) -> AppExit {
        // Build, finish, and cleanup the app plugins.
        {
            let plugins_state = app.plugins_state();
            if plugins_state != PluginsState::Cleaned {
                while app.plugins_state() == PluginsState::Adding {
                    bevy_tasks::tick_global_task_pools_on_main_thread();
                }
                app.finish();
                app.cleanup();
            }
        }

        // Collect all multi-threaded SubApps
        let mut sub_apps: Vec<(SubApp, InternedAppLabel)> = Vec::new();
        if let Some(labels) = app.world_mut().remove_resource::<ThreadedSubApps>() {
            sub_apps = labels
                .0
                .into_iter()
                .filter_map(|label| app.remove_sub_app(label).map(|sub| (sub, label)))
                .collect();
        }

        let pool = ComputeTaskPool::get();
        loop {
            pool.scope::<_, ()>(|scope| {
                #[cfg(feature = "trace")]
                let _threaded_update_span = info_span!("threaded update").entered();

                // Sync and spawn tasks for all multi-threaded SubApps.
                for (sub_app, _label) in &mut sub_apps {
                    {
                        #[cfg(feature = "trace")]
                        let _threaded_extract_span =
                            info_span!("threaded extract", name = ?_label).entered();
                        sub_app.extract(app.world_mut());

                        scope.spawn(async {
                            #[cfg(feature = "trace")]
                            let _threaded_sub_app_span =
                                info_span!("threaded sub app", name = ?_label.clone()).entered();

                            sub_app.update();
                        });
                    }
                }

                #[cfg(feature = "trace")]
                drop(_threaded_update_span);

                // Update the App and other SubApps while those process.
                app.update();
            });

            // Exit if requested.
            if let Some(exit) = app.should_exit() {
                return exit;
            }
        }
    }
}
