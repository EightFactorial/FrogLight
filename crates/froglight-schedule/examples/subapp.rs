//! TODO

use bevy::{MinimalPlugins, app::AppLabel, log::LogPlugin, prelude::*};
use froglight_schedule::{prelude::*, subapp::SubAppPlugin};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, AppLabel)]
struct Test;

fn main() -> AppExit {
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, LogPlugin::default()));
    app.add_plugins(SubAppPlugin::new(Test));

    // Exit the app after 1 second
    app.add_systems(Update, |time: Res<Time>, mut commands: Commands| {
        time.elapsed_secs().gt(&1.001).then(|| {
            commands.send_event(AppExit::Success);
        });
    });

    // Print the system order.
    {
        let sub = app.sub_app_mut(Test);
        **sub.world_mut().resource_mut::<TickRate>() = u32::MAX;

        // Execute ticks based on `TickRate`
        sub.add_systems(Main, trigger_tick.after(ShouldTick::update_tick).before(Main::run_main));

        sub.add_systems(First, (|| info!("First!")).run_if(run_once));
        sub.add_systems(Network::PreNetwork, (|| info!("PreNetwork!")).run_if(run_once));
        sub.add_systems(Tick::PreTick, (|| info!("PreTick!")).run_if(run_once));

        sub.add_systems(Tick::Tick, |tick: Res<CurrentTick>| {
            info!("Tick! ({})", **tick);
        });

        sub.add_systems(Tick::PostTick, (|| info!("PostTick!")).run_if(run_once));
        sub.add_systems(Network::PostNetwork, (|| info!("PostNetwork!")).run_if(run_once));
        sub.add_systems(Last, (|| info!("Last!")).run_if(run_once));
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
