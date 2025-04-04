use bevy::{MinimalPlugins, log::LogPlugin, prelude::*};

use super::{SchedulePlugin, TickSettings};
use crate::schedule::{PostNetwork, PostTick, PreNetwork, PreTick, Tick};

#[test]
fn app() -> AppExit {
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, LogPlugin::default(), SchedulePlugin));

    // Exit the app after 0.5 seconds
    app.add_systems(Update, |time: Res<Time>, mut commands: Commands| {
        time.elapsed_secs().gt(&0.5).then(|| {
            commands.send_event(AppExit::Success);
        });
    });

    // Print the system order.
    {
        app.add_systems(
            First,
            (|| info_once!("First!")).after(SchedulePlugin::tick_update).run_if(any_ticks),
        );
        app.add_systems(PreNetwork, || info_once!("PreNetwork!"));
        app.add_systems(PreUpdate, (|| info_once!("PreUpdate!")).run_if(any_ticks));
        app.add_systems(PreTick, || info_once!("PreTick!"));
        app.add_systems(Tick, || info!("Tick!"));
        app.add_systems(PostTick, || info_once!("PostTick!"));
        app.add_systems(Update, (|| info_once!("Update!")).run_if(any_ticks));
        app.add_systems(PostUpdate, (|| info_once!("PostUpdate!")).run_if(any_ticks));
        app.add_systems(PostNetwork, || info_once!("PostNetwork!"));
        app.add_systems(Last, (|| info_once!("Last!")).run_if(any_ticks));
    }

    app.run()
}

/// A [`Condition`] that checks if there are any ticks to run.
fn any_ticks(tick: Res<TickSettings>) -> bool { tick.ticks() > 0 }
