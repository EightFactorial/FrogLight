//! TODO

use alloc::vec::Vec;

use bevy_ecs::{
    entity::{EntityHashMap, EntityHashSet},
    prelude::*,
    schedule::ScheduleLabel,
};
use bevy_reflect::{Reflect, std_traits::ReflectDefault};
use bevy_time::{Real, Time};
#[cfg(feature = "froglight")]
use froglight_instance::prelude::SessionInstance;

use crate::{disable::TickDisabledSet, prelude::*};

/// A set of [`Schedule`]s used to control the order of ticking systems.
///
/// [`Schedule`]: bevy_ecs::schedule::Schedule
#[rustfmt::skip]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Reflect, ScheduleLabel)]
#[reflect(Debug, Default, Clone, PartialEq, PartialOrd, Hash)]
pub enum TickSchedule {
    /// Runs first in the tick schedule.
    ///
    /// Equivalent to [`First`](bevy_app::First).
    TickFirst,
    /// The schedule that contains logic that must run before [`Tick`](Self::Tick).
    ///
    /// Equivalent to [`PreUpdate`](bevy_app::PreUpdate).
    PreTick,
    /// The schedule that contains any tick logic.
    ///
    /// Equivalent to [`Update`](bevy_app::Update).
    #[default]
    Tick,
    /// The schedule that contains logic that must run after [`Tick`](Self::Tick).
    ///
    /// Equivalent to [`PostUpdate`](bevy_app::PostUpdate).
    PostTick,
    /// Runs last in the tick schedule.
    ///
    /// Equivalent to [`Last`](bevy_app::Last).
    TickLast,
}

// -------------------------------------------------------------------------------------------------

/// A [`Schedule`] that runs the [`TickSchedule`]s in order.
///
/// [`Schedule`]: bevy_ecs::schedule::Schedule
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect, ScheduleLabel)]
#[reflect(Debug, Default, Clone, PartialEq, Hash)]
pub struct RunTickLoop;

impl RunTickLoop {
    /// A [`System`] that runs the [`TickSchedule`]s in order.
    ///
    /// Ticks all [`TickTimer`]s and executes them the required number of ticks.
    ///
    /// Disabled [`TickTimer`]s and their children are temporarily marked
    /// [`TickDisabled`].
    pub fn run_tick(world: &mut World) {
        // Collect all ticking timers to keep enabled.
        let mut enabled = EntityHashMap::new();
        // Collect all non-ticking entities to disable.
        let mut disabled = EntityHashSet::with_capacity(enabled.len());

        // TODO: Set a maximum delta time and/or tick count.
        let delta = world.resource::<Time<Real>>().delta();
        for (entity, mut timer) in world.query::<(Entity, &mut TickTimer)>().iter_mut(world) {
            if timer.tick(delta).just_finished() {
                enabled.insert(entity, timer.times_finished_this_tick());
            }
        }

        // Get the maximum tick count of the timers (or return if none).
        let Some(ticks) = enabled.values().max() else { return };

        // Run the tick schedule for each tick.
        for iteration in 0..*ticks {
            // Disable all non-ticking entities.
            Self::insert_disable(iteration, &mut enabled, &mut disabled, world);
            // Stop if there are no enabled timers left.
            if enabled.is_empty() {
                break;
            }

            // Run all `TickSchedule`s.
            let _ = world.try_run_schedule(TickSchedule::TickFirst);
            let _ = world.try_run_schedule(TickSchedule::PreTick);
            let _ = world.try_run_schedule(TickSchedule::Tick);
            let _ = world.try_run_schedule(TickSchedule::PostTick);
            let _ = world.try_run_schedule(TickSchedule::TickLast);
        }

        // Re-enable all previously disabled entities.
        Self::remove_disable(world);
    }

    fn insert_disable(
        iteration: u32,
        enabled: &mut EntityHashMap<u32>,
        disabled: &mut EntityHashSet,
        world: &mut World,
    ) {
        // Remove finished timers from the `enabled` map.
        enabled.retain(|_, count| iteration < *count);
        // Stop if there are no enabled timers left.
        if enabled.is_empty() {
            return;
        }

        // Collect all non-ticking timers.
        let mut timers = Vec::new();
        for entity in world.query_filtered::<Entity, With<TickTimer>>().iter(world) {
            if !enabled.contains_key(&entity) {
                timers.push(entity);
            }
        }

        // Add all of the timers' children.
        disabled.extend(timers.iter().copied());
        for entity in timers.iter().filter_map(|e| world.get_entity(*e).ok()) {
            Self::insert_disable_timer(entity, disabled, world);
        }

        // Batch disable all non-ticking entities.
        world.resource_mut::<TickDisabledSet>().extend(disabled.iter().copied());
        world.commands().insert_batch(disabled.clone().into_iter().map(|e| (e, TickDisabled)));
    }

    fn insert_disable_timer(entity: EntityRef<'_>, disabled: &mut EntityHashSet, world: &World) {
        // Skip checking this timer if it is already disabled.
        if disabled.contains(&entity.id()) {
            return;
        }

        // Disable all entities in the `SessionInstance` if present.
        #[cfg(feature = "froglight")]
        if entity.contains::<SessionInstance>() {
            Self::insert_disable_instance(entity, disabled, world);
        }

        // Disable all children if present.
        if entity.contains::<Children>() {
            Self::insert_disable_children(entity, disabled, world);
        }
    }

    #[cfg(feature = "froglight")]
    fn insert_disable_instance(entity: EntityRef<'_>, disabled: &mut EntityHashSet, world: &World) {
        let Some(instance) = entity.get::<SessionInstance>() else { return };
        if instance.entity_count() == 0 {
            return;
        }

        // Disable all entities in the `SessionInstance`.
        disabled.extend(instance.iter_entity());

        // Disable all children of the entities in the `SessionInstance`.
        for entity in instance.iter_entity().filter_map(|e| world.get_entity(*e).ok()) {
            if entity.contains::<Children>() {
                Self::insert_disable_children(entity, disabled, world);
            }
        }
    }

    fn insert_disable_children(entity: EntityRef<'_>, disabled: &mut EntityHashSet, world: &World) {
        let Some(children) = entity.get::<Children>() else { return };
        if children.is_empty() {
            return;
        }

        // Disable all children of the entity.
        disabled.extend(children.iter());

        // Disable all children of the children.
        for entity in children.iter().filter_map(|e| world.get_entity(e).ok()) {
            Self::insert_disable_timer(entity, disabled, world);
        }
    }

    fn remove_disable(world: &mut World) {
        world.resource_scope::<TickDisabledSet, ()>(|world, mut disabled| {
            for entity in disabled.drain() {
                if let Ok(mut entity) = world.get_entity_mut(entity)
                    && entity.contains::<TickDisabled>()
                {
                    entity.remove::<TickDisabled>();
                }
            }
        });
    }
}
