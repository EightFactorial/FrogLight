//! Network and Tick [`ScheduleLabel`](bevy_ecs::schedule::ScheduleLabel)s

use std::marker::PhantomData;

use bevy_app::{MainScheduleOrder, prelude::*};
use bevy_ecs::{
    prelude::*,
    schedule::{ExecutorKind, InternedScheduleLabel, ScheduleLabel},
};

use crate::{prelude::*, systemset::SystemSetPlugin};

mod label;
pub use label::*;

/// A [`Plugin`] that adds network and tick
/// [`Schedule`](bevy_ecs::schedule::Schedule)s to an [`App`].
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
        app.init_schedule(PreNetwork);
        app.init_schedule(PreTick);
        app.init_schedule(Tick);
        app.init_schedule(PostTick);
        app.init_schedule(PostNetwork);

        // Add and initialize `CurrentTick`, `TickRate`, and `ShouldTick`.
        app.init_resource::<CurrentTick>().register_type::<CurrentTick>();
        app.init_resource::<TickRate>().register_type::<TickRate>();
        app.init_resource::<ShouldTick>().register_type::<ShouldTick>();

        app.add_systems(
            First,
            (ShouldTick::update_tick, CurrentTick::increment_tick.run_if(ShouldTick::should_tick))
                .chain(),
        );

        if let Some(mut order) = app.world_mut().get_resource_mut::<MainScheduleOrder>() {
            // Add `PreNetwork`
            if order.labels.contains(&PreUpdate.intern()) {
                // First, ..., PreNetwork, PreUpdate, ...
                order.insert_before(PreUpdate, OnTick::<PreNetwork>::default());
            } else if order.labels.contains(&First.intern()) {
                // First, PreNetwork, ...
                order.insert_after(First, OnTick::<PreNetwork>::default());
            } else {
                // ..., PreNetwork
                order.labels.push(OnTick::<PreNetwork>::default().intern());
            }

            // Add `PreTick`, `Tick`, and `PostTick`
            if order.labels.contains(&Update.intern()) {
                // ..., PreTick, Tick, PostTick, Update, ...
                order.insert_before(Update, OnTick::<PreTick>::default());
                order.insert_after(OnTick::<PreTick>::default(), OnTick::<Tick>::default());
                order.insert_after(OnTick::<Tick>::default(), OnTick::<PostTick>::default());
            } else {
                // ..., PreNetwork, PreTick, Tick, PostTick, ...
                order.insert_after(OnTick::<PreNetwork>::default(), OnTick::<PreTick>::default());
                order.insert_after(OnTick::<PreTick>::default(), OnTick::<Tick>::default());
                order.insert_after(OnTick::<Tick>::default(), OnTick::<PostTick>::default());
            }

            // Add `PostNetwork`
            if order.labels.contains(&PostUpdate.intern()) {
                // ..., PostUpdate, PostNetwork, ...
                order.insert_after(PostUpdate, OnTick::<PostNetwork>::default());
            } else if order.labels.contains(&Last.intern()) {
                // ..., PostNetwork, Last
                order.insert_before(Last, OnTick::<PostNetwork>::default());
            } else {
                // ..., PostNetwork
                order.insert_after(OnTick::<PostTick>::default(), OnTick::<PostNetwork>::default());
            }

            // Setup the `OnTick` schedules, systems and resources.
            OnTick::<PreNetwork>::setup(app, PreNetwork);
            OnTick::<PreTick>::setup(app, PreTick);
            OnTick::<Tick>::setup(app, Tick);
            OnTick::<PostTick>::setup(app, PostTick);
            OnTick::<PostNetwork>::setup(app, PostNetwork);
        }

        // Add the `SystemSetPlugin` to the app.
        app.add_plugins(SystemSetPlugin);
    }
}

// -------------------------------------------------------------------------------------------------

/// A wrapper [`ScheduleLabel`] for running schedules on every tick.
#[derive(Debug, ScheduleLabel)]
pub struct OnTick<Label: ScheduleLabel>(PhantomData<Label>);

/// A wrapper [`Resource`] for the [`OnTick`] schedule label.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Resource)]
struct OnTickLabel<Label: ScheduleLabel>(InternedScheduleLabel, PhantomData<Label>);

impl<Label: ScheduleLabel> OnTick<Label> {
    fn setup(app: &mut App, label: Label) {
        app.insert_resource(OnTickLabel::<Label>(label.intern(), PhantomData));

        let mut schedule = Schedule::new(label);
        schedule.set_executor_kind(ExecutorKind::SingleThreaded);
        app.add_schedule(schedule);

        app.add_systems(
            OnTick::<Label>::default(),
            OnTick::<Label>::execute_schedule.run_if(ShouldTick::should_tick),
        );
    }

    /// A [`System`] that runs the `Label` schedule
    /// for the amount of ticks that need to be run.
    fn execute_schedule(world: &mut World) {
        world.run_schedule(world.resource::<OnTickLabel<Label>>().0);
    }
}

// -------------------------------------------------------------------------------------------------

impl<Label: ScheduleLabel> Default for OnTick<Label> {
    fn default() -> Self { Self(PhantomData) }
}

impl<Label: ScheduleLabel> Clone for OnTick<Label> {
    #[allow(clippy::non_canonical_clone_impl)]
    fn clone(&self) -> Self { Self(PhantomData) }
}
impl<Label: ScheduleLabel> Copy for OnTick<Label> {}

impl<Label: ScheduleLabel> PartialEq for OnTick<Label> {
    fn eq(&self, other: &Self) -> bool { self.0 == other.0 }
}
impl<Label: ScheduleLabel> Eq for OnTick<Label> {}

impl<Label: ScheduleLabel> std::hash::Hash for OnTick<Label> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) { self.0.hash(state) }
}
