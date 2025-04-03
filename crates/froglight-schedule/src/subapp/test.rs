use bevy::{MinimalPlugins, app::AppLabel, log::LogPlugin, prelude::*};

use super::SubAppPlugin;
use crate::schedule::{PostNetwork, PostTick, PreNetwork, PreTick, Tick};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, AppLabel)]
struct Test;

#[test]
fn app() -> AppExit {
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, LogPlugin::default()));
    app.add_plugins(SubAppPlugin::new(Test));

    // Exit the app after 0.5 seconds
    app.add_systems(Update, |time: Res<Time>, mut commands: Commands| {
        time.elapsed_secs().gt(&0.5).then(|| {
            commands.send_event(AppExit::Success);
        });
    });

    // Print the system order.
    {
        let sub = app.sub_app_mut(Test);
        sub.add_systems(First, || info_once!("First!"));
        sub.add_systems(PreNetwork, || info_once!("PreNetwork!"));
        sub.add_systems(PreTick, || info_once!("PreTick!"));
        sub.add_systems(Tick, || info!("Tick!"));
        sub.add_systems(PostTick, || info_once!("PostTick!"));
        sub.add_systems(PostNetwork, || info_once!("PostNetwork!"));
        sub.add_systems(Last, || info_once!("Last!"));
    }

    app.run()
}
