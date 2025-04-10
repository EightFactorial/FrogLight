use bevy_app::{AppLabel, prelude::*};
use bevy_ecs::prelude::*;
use derive_more::{Deref, DerefMut};

#[derive(Debug, Default, Clone, Resource, Deref, DerefMut)]
pub(super) struct ThreadedSubApps(Vec<bevy_app::InternedAppLabel>);

impl ThreadedSubApps {
    pub(super) fn runner(mut app: App) -> AppExit {
        use bevy_app::PluginsState;

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

        // Collect all threaded SubApps
        let mut sub_apps: Vec<SubApp> = Vec::new();
        if let Some(labels) = app.world_mut().remove_resource::<ThreadedSubApps>() {
            labels.iter().for_each(|l| sub_apps.push(app.remove_sub_app(l.intern()).unwrap()));
        }

        // TODO: Also spawn a task for `app.update()`
        let pool = bevy_tasks::ComputeTaskPool::get();
        loop {
            pool.scope::<_, ()>(|scope| {
                // Sync and spawn tasks for all multi-threaded SubApps.
                for sub_app in &mut sub_apps {
                    sub_app.extract(app.world_mut());
                    scope.spawn(async { sub_app.update() });
                }

                // Update the App and single-threaded SubApps locally.
                app.update();
            });

            // Exit if requested.
            match app.should_exit() {
                None => (),
                Some(exit) => return exit,
            }
        }
    }
}
