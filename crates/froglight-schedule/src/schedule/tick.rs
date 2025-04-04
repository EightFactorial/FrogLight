use std::{
    sync::atomic::{AtomicBool, Ordering},
    time::Duration,
};

use bevy_ecs::prelude::*;
use bevy_reflect::prelude::*;
use derive_more::{Deref, DerefMut};

/// A counter for the current tick.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Resource, Reflect, Deref, DerefMut)]
#[reflect(Debug, Default, PartialEq, Hash, Resource)]
pub struct CurrentTick(u128);

impl CurrentTick {
    /// A [`System`] that increments the [`CurrentTick`].
    pub fn increment_tick(mut tick: ResMut<Self>) { tick.increment(); }

    /// Increment the [`CurrentTick`].
    pub fn increment(&mut self) { self.0 = self.0.wrapping_add(1); }
}

// -------------------------------------------------------------------------------------------------

/// How many ticks should be executed per second.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Resource, Reflect)]
#[reflect(Debug, Default, PartialEq, Hash, Resource)]
pub struct TickRate(u32);

impl Default for TickRate {
    fn default() -> Self { Self(20) }
}

impl TickRate {
    /// Get the expected number of ticks per second.
    #[inline]
    #[must_use]
    pub const fn per_second(&self) -> u32 { self.0 }

    /// Get the expected number of seconds between ticks.
    #[inline]
    #[must_use]
    pub const fn duration_f64(&self) -> f64 {
        match self.0 {
            0 => 0f64,
            x => (x as f64).recip(),
        }
    }

    /// Get the expected [`Duration`] between ticks.
    #[must_use]
    pub fn duration(&self) -> Duration { Duration::from_secs_f64(self.duration_f64()) }
}

// -------------------------------------------------------------------------------------------------

/// Whether a tick should be executed during the next update.
#[derive(Debug, Default, Resource, Reflect)]
#[reflect(Debug, Default, Resource)]
pub struct ShouldTick {
    current: AtomicBool,
    next: AtomicBool,
}

impl ShouldTick {
    /// Get whether a tick should execute this update.
    #[must_use]
    pub fn get_current(&self) -> bool { self.current.load(Ordering::Relaxed) }

    /// Get whether a tick should execute next update.
    #[must_use]
    pub fn get_next(&self) -> bool { self.next.load(Ordering::Relaxed) }

    /// Trigger a tick next update.
    pub fn set_next(&mut self) { self.next.store(true, Ordering::Relaxed); }

    /// Cancel a tick from running next update.
    pub fn clear_next(&mut self) { self.next.store(false, Ordering::Relaxed); }

    /// A [`Condition`] that returns `true` if a tick should execute.
    #[must_use]
    pub fn should_tick(tick: Res<Self>) -> bool { tick.get_current() }

    /// A [`System`] that updates [`ShouldTick`] at the start of a frame.
    pub fn update_tick(mut tick: ResMut<Self>) {
        let Self { current, next } = &mut *tick;
        std::mem::swap(current, next);
        next.store(false, Ordering::Relaxed);
    }
}
