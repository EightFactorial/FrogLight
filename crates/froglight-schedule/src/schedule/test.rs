use bevy::{MinimalPlugins, log::LogPlugin, prelude::*};

use super::SchedulePlugin;
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
        app.add_systems(First, || info_once!("First!"));
        app.add_systems(PreNetwork, || info_once!("PreNetwork!"));
        app.add_systems(PreUpdate, || info_once!("PreUpdate!"));
        app.add_systems(PreTick, || info_once!("PreTick!"));
        app.add_systems(Tick, || info!("Tick!"));
        app.add_systems(PostTick, || info_once!("PostTick!"));
        app.add_systems(Update, || info_once!("Update!"));
        app.add_systems(PostUpdate, || info_once!("PostUpdate!"));
        app.add_systems(PostNetwork, || info_once!("PostNetwork!"));
        app.add_systems(Last, || info_once!("Last!"));
    }

    app.run()
}
