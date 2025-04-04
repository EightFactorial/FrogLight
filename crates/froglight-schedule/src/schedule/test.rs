use bevy::{MinimalPlugins, log::LogPlugin, prelude::*};

use super::SchedulePlugin;
use crate::prelude::*;

#[test]
fn app() -> AppExit {
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, LogPlugin::default(), SchedulePlugin));

    // Exit the app after 0.5 seconds
    app.add_systems(Update, |time: Res<Time>, mut commands: Commands| {
        time.elapsed_secs().gt(&0.501).then(|| {
            commands.send_event(AppExit::Success);
        });
    });

    // Execute ticks based on `TickRate`
    app.add_systems(First, trigger_tick.after(bevy::time::time_system));

    // Print the system order.
    {
        app.add_systems(
            First,
            (|| info_once!("First!"))
                .after(ShouldTick::update_tick)
                .run_if(ShouldTick::should_tick),
        );
        app.add_systems(PreNetwork, || info_once!("PreNetwork!"));
        app.add_systems(PreUpdate, (|| info_once!("PreUpdate!")).run_if(ShouldTick::should_tick));
        app.add_systems(PreTick, || info_once!("PreTick!"));

        app.add_systems(Tick, |tick: Res<CurrentTick>| {
            info!("Tick! ({})", **tick);
        });

        app.add_systems(PostTick, || info_once!("PostTick!"));
        app.add_systems(Update, (|| info_once!("Update!")).run_if(ShouldTick::should_tick));
        app.add_systems(PostUpdate, (|| info_once!("PostUpdate!")).run_if(ShouldTick::should_tick));
        app.add_systems(PostNetwork, || info_once!("PostNetwork!"));
        app.add_systems(Last, (|| info_once!("Last!")).run_if(ShouldTick::should_tick));
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
