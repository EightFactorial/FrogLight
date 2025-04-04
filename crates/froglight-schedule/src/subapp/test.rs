use bevy::{MinimalPlugins, app::AppLabel, log::LogPlugin, prelude::*};

use super::SubAppPlugin;
use crate::prelude::*;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, AppLabel)]
struct Test;

#[test]
fn app() -> AppExit {
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, LogPlugin::default()));
    app.add_plugins(SubAppPlugin::new(Test));

    // Exit the app after 0.5 seconds
    app.add_systems(Update, |time: Res<Time>, mut commands: Commands| {
        time.elapsed_secs().gt(&0.501).then(|| {
            commands.send_event(AppExit::Success);
        });
    });

    // Print the system order.
    {
        let sub = app.sub_app_mut(Test);

        // Execute ticks based on `TickRate`
        sub.add_systems(Main, trigger_tick.after(ShouldTick::update_tick));

        sub.add_systems(First, || info_once!("First!"));
        sub.add_systems(PreNetwork, || info_once!("PreNetwork!"));
        sub.add_systems(PreTick, || info_once!("PreTick!"));

        sub.add_systems(Tick, |tick: Res<CurrentTick>| {
            info!("Tick! ({})", **tick);
        });

        sub.add_systems(PostTick, || info_once!("PostTick!"));
        sub.add_systems(PostNetwork, || info_once!("PostNetwork!"));
        sub.add_systems(Last, || info_once!("Last!"));
    }

    app.run()
}

/// Execute ticks based on `TickRate`.
fn trigger_tick(
    rate: Res<TickRate>,
    time: Res<Time<Real>>,
    mut should: ResMut<ShouldTick>,
    mut timer: Local<Option<Timer>>,
) {
    let timer = timer.get_or_insert_with(|| Timer::new(rate.duration(), TimerMode::Repeating));

    if timer.tick(time.delta()).just_finished() {
        should.set_next();
    }
}
