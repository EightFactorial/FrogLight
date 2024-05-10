use std::{marker::PhantomData, time::Duration};

use bevy_derive::{Deref, DerefMut};
use bevy_ecs::{reflect::ReflectResource, system::Resource};
use bevy_reflect::{std_traits::ReflectDefault, Reflect};

#[derive(Debug, Default, PartialEq, Eq, Hash, Deref, DerefMut, Resource, Reflect)]
#[reflect(Default, Resource)]
pub(super) struct ScheduleTimer<T: Default> {
    #[deref]
    duration: Duration,
    #[reflect(ignore)]
    _marker: PhantomData<T>,
}

impl<T: Default> ScheduleTimer<T> {
    /// Adds the given duration to the timer.
    pub(super) fn tick(&mut self, delta: Duration) { self.duration += delta; }

    /// Returns the number of runs that should be executed.
    ///
    /// This method will subtract the maximum number of runs that can be
    /// executed from the current duration and return that number.
    #[must_use]
    #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
    pub(super) fn runs(&mut self, interval: Duration) -> u32 {
        let runs = (self.duration.as_secs_f64() / interval.as_secs_f64()).floor() as u32;
        self.duration = self.duration.saturating_sub(interval * runs);
        runs
    }
}
