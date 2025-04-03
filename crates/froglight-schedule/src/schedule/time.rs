use std::time::Duration;

use bevy_ecs::prelude::*;
use bevy_reflect::prelude::*;
use bevy_time::{Real, Time, Timer, TimerMode};

/// Settings for how ticks should be executed.
#[derive(Debug, Clone, PartialEq, Eq, Resource, Reflect)]
#[reflect(Debug, Default, PartialEq, Resource)]
pub struct TickSettings {
    /// The current tick.
    current: u128,
    /// The tick timer.
    timer: Timer,

    /// The target amount of ticks per second.
    target: u32,
    /// The maximum number of ticks to run per frame.
    max_per_frame: u32,

    /// The amount of ticks to catch up to.
    remaining: u32,
    /// The amount of ticks that will run this frame.
    should_run: u32,
}

impl Default for TickSettings {
    fn default() -> Self {
        Self {
            current: 0,
            remaining: 0,
            should_run: 0,
            target: TickSettings::DEFAULT_TPS,
            max_per_frame: TickSettings::DEFAULT_MAX_PER_FRAME,
            #[expect(clippy::cast_precision_loss)]
            timer: Timer::from_seconds(
                1.0 / TickSettings::DEFAULT_TPS as f32,
                TimerMode::Repeating,
            ),
        }
    }
}

impl TickSettings {
    /// The default maximum amount of ticks per frame.
    pub const DEFAULT_MAX_PER_FRAME: u32 = 5;
    /// The default amount of ticks per second.
    pub const DEFAULT_TPS: u32 = 20;

    /// The current tick.
    #[inline]
    #[must_use]
    pub const fn current_tick(&self) -> u128 { self.current }

    /// The amount of ticks that need to be run to catch up to real time.
    #[inline]
    #[must_use]
    pub const fn ticks_behind(&self) -> u32 { self.remaining }

    /// The timer that is used to track the next tick.
    #[inline]
    #[must_use]
    pub const fn timer(&self) -> &Timer { &self.timer }

    /// The target amount of ticks per second.
    #[inline]
    #[must_use]
    pub const fn target(&self) -> u32 { self.target }

    /// The maximum amount of ticks that can be run per frame.
    #[inline]
    #[must_use]
    pub const fn maximum(&self) -> u32 { self.max_per_frame }

    /// The amount of ticks that will be run this frame.
    #[inline]
    #[must_use]
    pub const fn ticks(&self) -> u32 { self.should_run }

    /// Set the target amount of ticks to run per second.
    pub fn set_target(&mut self, tps: u32) {
        self.timer.set_duration(Duration::from_secs_f64(1.0 / f64::from(tps)));
        self.target = tps;
    }

    /// Set the maximum amount of ticks that can be run per frame.
    #[inline]
    pub fn set_maximum(&mut self, max: u32) { self.max_per_frame = max; }

    /// Update the [`TickSettings`] [`Timer`].
    pub fn update(&mut self, time: &Time<Real>) {
        // Update the current tick.
        self.current = self.current.wrapping_add(u128::from(self.should_run));

        // Update the timer.
        self.timer.tick(time.delta());
        if self.timer.just_finished() {
            // Add the amount of ticks that have passed
            self.remaining = self.remaining.saturating_add(self.timer.times_finished_this_tick());
            self.timer.set_elapsed(self.timer.elapsed());
        }

        // Set the amount of ticks that will run this frame
        self.should_run = self.remaining.min(self.max_per_frame);
        // Subtract the amount of tick that will be run.
        self.remaining = self.remaining.saturating_sub(self.should_run);
    }
}
