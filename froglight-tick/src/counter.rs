//! TODO

use bevy_ecs::{entity::EntityHashMap, prelude::*};
use bevy_reflect::{Reflect, std_traits::ReflectDefault};

use crate::prelude::*;

/// A counter for the number of ticks executed.
#[repr(transparent)]
#[derive(Debug, Default, Clone, PartialEq, Eq, Resource, Reflect)]
#[reflect(Debug, Default, Clone, PartialEq, Resource)]
pub struct TickCounter {
    per_timer: EntityHashMap<u128>,
}

impl TickCounter {
    /// Create a new [`TickCounter`].
    #[must_use]
    pub const fn new() -> Self { Self { per_timer: EntityHashMap::new() } }

    /// Get the number of ticks executed for a specific [`TickTimer`].
    ///
    /// [`TickTimer`]: crate::prelude::TickTimer
    #[inline]
    #[must_use]
    pub fn for_timer(&self, entity: &Entity) -> Option<u128> { self.per_timer.get(entity).copied() }

    /// Increment the number of ticks executed for the given entities.
    #[inline]
    pub fn increment(&mut self, entities: &[Entity]) {
        self.increment_iter(entities.iter().copied());
    }

    /// Increment the number of ticks executed for the given entities.
    pub fn increment_iter<T: Iterator<Item = Entity>>(&mut self, iter: T) {
        iter.for_each(|entity| {
            self.per_timer.entry(entity).and_modify(|val| *val = val.wrapping_add(1)).or_insert(1);
        });
    }

    /// Retains only the elements specified by the predicate. Keeps the
    /// allocated memory for reuse.
    #[inline]
    pub fn retain<F: FnMut(&Entity, &mut u128) -> bool>(&mut self, f: F) {
        self.per_timer.retain::<F>(f);
    }
}

// -------------------------------------------------------------------------------------------------

impl TickCounter {
    /// A [`System`] that increments the [`TickCounter`] [`Resource`].
    pub fn increment_counter(
        query: Query<Entity, With<TickTimer>>,
        mut counter: ResMut<TickCounter>,
    ) {
        counter.retain(|k, _| query.contains(*k));
        counter.increment_iter(query.into_iter());
    }
}
