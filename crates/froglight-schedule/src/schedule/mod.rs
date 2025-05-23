//! [`SchedulePlugin`] and [`SystemSetPlugin`]

use core::{fmt::Debug, hash::Hash};

use bevy_app::{MainScheduleOrder, prelude::*};
use bevy_ecs::{
    prelude::*,
    schedule::{ExecutorKind, ScheduleLabel},
};

use crate::{
    subworld::SubWorlds,
    tick::{CurrentTick, ShouldTick, TickRate},
};

pub mod label;
use label::{Network, Tick};

mod set;
pub use set::*;

/// A [`Plugin`] that adds network and tick [`Schedule`]s to an [`App`].
///
/// In an app with the default schedules, the order looks like (abbreviated):
/// - First, PreNet, PreUpd, PreT, Tick, PostT, Update, PostUpd, PostNet, Last
///
/// In an app with `First` and `Last`, the order looks like:
/// - First, PreNetwork, PreTick, Tick, PostTick, ..., PostNetwork, Last
///
/// In an app without any known schedules, the order looks like:
/// - ..., PreNetwork, PreTick, Tick, PostTick, PostNetwork
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SchedulePlugin;

impl Plugin for SchedulePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<SubWorlds>();

        app.init_schedule(Network::PreNetwork);
        app.init_schedule(Tick::PreTick);
        app.init_schedule(Tick::Tick);
        app.init_schedule(Tick::PostTick);
        app.init_schedule(Network::PostNetwork);

        // Add and initialize `CurrentTick`, `TickRate`, and `ShouldTick`.
        app.init_resource::<CurrentTick>().register_type::<CurrentTick>();
        app.init_resource::<TickRate>().register_type::<TickRate>();
        app.init_resource::<ShouldTick>().register_type::<ShouldTick>();

        app.add_systems(
            First,
            (ShouldTick::update, CurrentTick::increment_tick.run_if(ShouldTick::tick)).chain(),
        );

        if let Some(mut order) = app.world_mut().get_resource_mut::<MainScheduleOrder>() {
            let pre_network = OnTick(Network::PreNetwork);
            let pre_tick = OnTick(Tick::PreTick);
            let tick = OnTick(Tick::Tick);
            let post_tick = OnTick(Tick::PostTick);
            let post_network = OnTick(Network::PostNetwork);

            // Add `Network::PreNetwork`
            if order.labels.contains(&PreUpdate.intern()) {
                // First, ..., Network::PreNetwork, PreUpdate, ...
                order.insert_before(PreUpdate, pre_network);
            } else if order.labels.contains(&First.intern()) {
                // First, Network::PreNetwork, ...
                order.insert_after(First, pre_network);
            } else {
                // ..., Network::PreNetwork
                order.labels.push(pre_network.intern());
            }

            // Add `Tick::PreTick`, `Tick::Tick`, and `Tick::PostTick`
            if order.labels.contains(&Update.intern()) {
                // ..., PreTick, Tick, PostTick, Update, ...
                order.insert_before(Update, pre_tick);
            } else {
                // ..., Network::PreNetwork, Tick::PreTick, Tick::Tick, Tick::PostTick, ...
                order.insert_after(pre_network, pre_tick);
            }
            order.insert_after(pre_tick, tick);
            order.insert_after(tick, post_tick);

            // Add `Network::PostNetwork`
            if order.labels.contains(&PostUpdate.intern()) {
                // ..., PostUpdate, Network::PostNetwork, ...
                order.insert_after(PostUpdate, post_network);
            } else if order.labels.contains(&Last.intern()) {
                // ..., Network::PostNetwork, Last
                order.insert_before(Last, post_network);
            } else {
                // ..., Tick::PostTick, Network::PostNetwork
                order.insert_after(post_tick, post_network);
            }

            // Setup the `OnTick` schedules, systems and resources.
            OnTick::setup(app, Network::PreNetwork);
            OnTick::setup(app, Tick::PreTick);
            OnTick::setup(app, Tick::Tick);
            OnTick::setup(app, Tick::PostTick);
            OnTick::setup(app, Network::PostNetwork);
        }

        // Add the `SystemSetPlugin` to the app.
        app.add_plugins(SystemSetPlugin);
    }
}

// -------------------------------------------------------------------------------------------------

/// A wrapper [`ScheduleLabel`] for running schedules on every tick.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, ScheduleLabel)]
pub struct OnTick<Label: Debug + Copy + Eq + Hash + ScheduleLabel>(Label);

impl<Label: Debug + Copy + Eq + Hash + ScheduleLabel> OnTick<Label> {
    fn setup(app: &mut App, label: Label) {
        // Create a new `Schedule` for the label.
        let mut schedule = Schedule::new(label);
        schedule.set_executor_kind(ExecutorKind::SingleThreaded);

        // Execute the schedule with the provided label.
        let execute = move |world: &mut World| {
            world.run_schedule(label);
        };

        app.add_schedule(schedule)
            .add_systems(OnTick::<Label>(label), execute.run_if(ShouldTick::tick));
    }
}
