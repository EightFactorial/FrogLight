//! TODO

// use bevy_app::{First, Last, PostUpdate, PreUpdate, Update};
use bevy_ecs::{prelude::*, schedule::ScheduleLabel};
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

impl TickSchedule {
    /// A [`System`] that adds [`TickDisabled`] to entities whose [`TickTimer`]
    /// **has not** finished ticking.
    ///
    /// Runs during [`TickSchedule::TickFirst`].
    #[cfg(not(feature = "froglight"))]
    pub fn tickfirst_disable(
        query: Query<(Entity, &mut TickTimer)>,
        children: Query<&Children>,
        time: Res<Time<Real>>,
        mut disabled: ResMut<TickDisabledSet>,
        mut commands: Commands,
    ) {
        for (entity, mut timer) in query {
            if timer.tick(time.delta()).just_finished() {
                #[cfg(feature = "tracing")]
                tracing::trace!(target: "froglight_tick", "Ticking Entity {entity}");
            } else {
                // Disable the ticked entity.
                disabled.insert(entity);
                commands.entity(entity).insert(TickDisabled);

                // Disable all the entity's children.
                Self::disable_children(entity, &children, &mut disabled, &mut commands);
            }
        }
    }

    /// A [`System`] that adds [`TickDisabled`] to entities whose [`TickTimer`]
    /// **has not** finished ticking.
    ///
    /// Runs during [`TickSchedule::TickFirst`].
    #[cfg(feature = "froglight")]
    pub fn tickfirst_disable(
        query: Query<(Entity, Option<&SessionInstance>, &mut TickTimer)>,
        children: Query<&Children>,
        time: Res<Time<Real>>,
        mut disabled: ResMut<TickDisabledSet>,
        mut commands: Commands,
    ) {
        for (entity, instance, mut timer) in query {
            if timer.tick(time.delta()).just_finished() {
                #[cfg(feature = "tracing")]
                tracing::trace!(target: "froglight_tick", "Ticking Entity {entity}");
            } else {
                // Disable the ticked entity.
                disabled.insert(entity);
                commands.entity(entity).insert(TickDisabled);

                // Disable all the entity's children.
                Self::disable_children(entity, &children, &mut disabled, &mut commands);

                // If the entity has a [`SessionInstance`],
                // disable all the entities in the instance as well.
                if let Some(instance) = instance {
                    disabled.extend(instance.iter_entity().copied());
                    // Disable all the children of the entities in the instance as well.
                    for entity in instance.iter_entity() {
                        Self::disable_children(*entity, &children, &mut disabled, &mut commands);
                    }

                    let batch: alloc::vec::Vec<_> =
                        instance.iter_entity().map(|e| (*e, TickDisabled)).collect();
                    commands.insert_batch(batch);
                }
            }
        }
    }

    /// Recursively disables all children of the given entity.
    ///
    /// Batches [`TickDisabled`] insertions.
    fn disable_children(
        entity: Entity,
        children: &Query<&Children>,
        disabled: &mut TickDisabledSet,
        commands: &mut Commands,
    ) {
        // Skip if the entity has no children.
        if !children.contains(entity) {
            return;
        }

        // Collect all descendants that are not already disabled.
        let descendants: alloc::vec::Vec<_> =
            children.iter_descendants(entity).filter(|e| !disabled.contains(e)).collect();
        // Skip if there are no descendants to disable.
        if descendants.is_empty() {
            return;
        }

        // Register all the descendants as disabled.
        disabled.extend(descendants.iter().copied());
        // Batch insert `TickDisabled`.
        let batch: alloc::vec::Vec<_> = descendants.iter().map(|e| (*e, TickDisabled)).collect();
        commands.insert_batch(batch);

        // Recursively disable all descendants of all descendants.
        for entity in descendants.into_iter().filter(|e| children.contains(*e)) {
            Self::disable_children(entity, children, disabled, commands);
        }
    }

    /// A [`System`] that removes [`TickDisabled`] from entities disabled by
    /// [`TickSchedule::tickfirst_disable`].
    ///
    /// Runs during [`TickSchedule::TickLast`].
    pub fn ticklast_reenable(world: &mut World) {
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
    /// Silently skips any [`Schedule`]s
    /// that are not present (have no systems).
    pub fn run_tick(world: &mut World) {
        let _ = world.try_run_schedule(TickSchedule::TickFirst);
        let _ = world.try_run_schedule(TickSchedule::PreTick);
        let _ = world.try_run_schedule(TickSchedule::Tick);
        let _ = world.try_run_schedule(TickSchedule::PostTick);
        let _ = world.try_run_schedule(TickSchedule::TickLast);
    }
}
