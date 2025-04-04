//! Network and Tick [`ScheduleLabel`](bevy_ecs::schedule::ScheduleLabel)s

use std::marker::PhantomData;

use bevy_app::{MainScheduleOrder, prelude::*};
use bevy_ecs::{
    prelude::*,
    schedule::{ExecutorKind, InternedScheduleLabel, ScheduleLabel},
};
use bevy_time::{Real, Time};

use crate::systemset::SystemSetPlugin;

mod label;
pub use label::*;

mod tick;
pub use tick::*;

#[cfg(test)]
mod test;

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
        // Add and initialize `TickSettings`.
        app.init_resource::<TickSettings>().register_type::<TickSettings>();

        app.init_schedule(PreNetwork);
        app.init_schedule(PreTick);
        app.init_schedule(Tick);
        app.init_schedule(PostTick);
        app.init_schedule(PostNetwork);

        // Update `TickSettings` at the start of every frame.
        app.add_systems(First, Self::tick_update.after(bevy_time::time_system));

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

impl SchedulePlugin {
    pub(crate) fn tick_update(mut tick: ResMut<TickSettings>, time: Res<Time<Real>>) {
        tick.update(&time);
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
            OnTick::<Label>::execute_schedule.run_if(Self::any_ticks),
        );
    }

    /// A [`Condition`] that checks if there are any ticks to run.
    fn any_ticks(tick: Res<TickSettings>) -> bool { tick.ticks() > 0 }

    /// A [`System`] that runs the `Label` schedule
    /// for the amount of ticks that need to be run.
    fn execute_schedule(world: &mut World) {
        let ticks = world.resource::<TickSettings>().ticks();
        let label = world.resource::<OnTickLabel<Label>>().0;
        (0..ticks).for_each(|_| world.run_schedule(label));
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
