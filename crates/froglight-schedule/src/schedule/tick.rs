use std::time::Duration;

use bevy_ecs::prelude::*;
use bevy_reflect::prelude::*;
use bevy_time::{Real, Time, Timer, TimerMode};

/// Settings for how ticks should be executed.
#[derive(Debug, Clone, PartialEq, Eq, Resource, Reflect)]
#[reflect(Debug, Default, PartialEq, Resource)]
pub struct TickSettings {
    /// The tick timer.
    timer: Timer,

    /// The target amount of ticks per second.
    target: u32,
    /// The maximum number of ticks to run per frame.
    maximum: u32,

    /// The tick this frame started at.
    start: u128,
    /// The amount of ticks that will run this frame.
    running: u32,
    /// The amount of ticks to catch up to real time.
    remaining: u32,
}

impl Default for TickSettings {
    fn default() -> Self {
        Self {
            start: 0,
            running: 0,
            remaining: 0,
            target: TickSettings::DEFAULT_TPS,
            maximum: TickSettings::DEFAULT_MAX_PER_FRAME,
            timer: Timer::new(
                Duration::from_secs_f64(1.0 / f64::from(TickSettings::DEFAULT_TPS)),
                TimerMode::Repeating,
            ),
        }
    }
}

impl TickSettings {
    /// The default maximum amount of ticks per frame.
    pub const DEFAULT_MAX_PER_FRAME: u32 = 5;
    /// The default number of ticks per second.
    pub const DEFAULT_TPS: u32 = 20;

    /// The amount of ticks that will be run this frame.
    #[inline]
    #[must_use]
    pub const fn ticks(&self) -> u32 { self.running }

    /// The tick this frame started at.
    #[inline]
    #[must_use]
    pub const fn ticks_start(&self) -> u128 { self.start }

    /// The amount of ticks needed to catch up to real time.
    #[inline]
    #[must_use]
    pub const fn ticks_behind(&self) -> u32 { self.remaining }

    /// The timer used to track the next tick.
    #[inline]
    #[must_use]
    pub const fn timer(&self) -> &Timer { &self.timer }

    /// The target amount of ticks per second.
    #[inline]
    #[must_use]
    pub const fn target(&self) -> u32 { self.target }

    /// Set the target amount of ticks to run per second.
    pub fn set_target(&mut self, tps: u32) {
        self.timer.set_duration(Duration::from_secs_f64(1.0 / f64::from(tps)));
        self.target = tps;
    }

    /// The maximum amount of ticks that can be run per frame.
    #[inline]
    #[must_use]
    pub const fn maximum(&self) -> u32 { self.maximum }

    /// Set the maximum amount of ticks that can be run per frame.
    #[inline]
    pub fn set_maximum(&mut self, max: u32) { self.maximum = max; }

    /// Update the [`TickSettings`] [`Timer`].
    pub fn update(&mut self, time: &Time<Real>) {
        // Update the current tick.
        self.start = self.start.wrapping_add(u128::from(self.running));

        // Update the timer.
        self.timer.tick(time.delta());
        if self.timer.just_finished() {
            // Add the amount of ticks that have passed
            self.remaining = self.remaining.saturating_add(self.timer.times_finished_this_tick());
            self.timer.set_elapsed(self.timer.elapsed());
        }

        // Set the amount of ticks that will run this frame
        self.running = self.remaining.min(self.maximum);
        // Subtract the amount of tick that will be run.
        self.remaining = self.remaining.saturating_sub(self.running);
    }
}
