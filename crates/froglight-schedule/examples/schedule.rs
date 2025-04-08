//! TODO

use bevy::{MinimalPlugins, log::LogPlugin, prelude::*};
use froglight_schedule::{prelude::*, schedule::SchedulePlugin};

fn main() -> AppExit {
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, LogPlugin::default(), SchedulePlugin));

    // Exit the app after 1 second
    app.add_systems(Update, |time: Res<Time>, mut commands: Commands| {
        time.elapsed_secs().gt(&1.001).then(|| {
            commands.send_event(AppExit::Success);
        });
    });

    // Execute ticks based on `TickRate`
    app.add_systems(First, trigger_tick.after(bevy::time::time_system));
    **app.world_mut().resource_mut::<TickRate>() = u32::MAX;

    // Print the system order.
    {
        app.add_systems(
            First,
            (|| info!("First!"))
                .after(ShouldTick::update_tick)
                .run_if(ShouldTick::should_tick)
                .run_if(run_once),
        );
        app.add_systems(
            PreUpdate,
            (|| info!("PreUpdate!")).run_if(ShouldTick::should_tick).run_if(run_once),
        );
        app.add_systems(
            Update,
            (|| info!("Update!")).run_if(ShouldTick::should_tick).run_if(run_once),
        );
        app.add_systems(
            PostUpdate,
            (|| info!("PostUpdate!")).run_if(ShouldTick::should_tick).run_if(run_once),
        );
        app.add_systems(Last, (|| info!("Last!")).run_if(ShouldTick::should_tick).run_if(run_once));

        app.add_systems(PreNetwork, (|| info!("PreNetwork!")).run_if(run_once));
        app.add_systems(PreTick, (|| info!("PreTick!")).run_if(run_once));

        app.add_systems(Tick, |tick: Res<CurrentTick>| {
            info!("Tick! ({})", **tick);
        });

        app.add_systems(PostTick, (|| info!("PostTick!")).run_if(run_once));
        app.add_systems(PostNetwork, (|| info!("PostNetwork!")).run_if(run_once));
    }

    app.run()
}

/// Execute ticks based on `TickRate`.
fn trigger_tick(
    rate: Res<TickRate>,
    time: Res<Time<Real>>,
    should: ResMut<ShouldTick>,
    mut timer: Local<Option<Timer>>,
) {
    let timer = timer.get_or_insert_with(|| Timer::new(rate.duration(), TimerMode::Repeating));

    if timer.tick(time.delta()).just_finished() {
        should.set_next();
    }
}
