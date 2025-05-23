//! [`ScheduleLabel`]s and [`SystemSet`](bevy_ecs::schedule::SystemSet)s

use bevy_ecs::schedule::ScheduleLabel;

/// The [`Network::PreNetwork`] and [`Network::PostNetwork`] schedules.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, ScheduleLabel)]
pub enum Network {
    /// A [`Schedule`](bevy_ecs::schedule::Schedule) for early-frame networking.
    ///
    /// Typically used for receiving network messages.
    PreNetwork,
    /// A [`Schedule`](bevy_ecs::schedule::Schedule) for late-frame networking.
    ///
    /// Typically used for sending network messages.
    PostNetwork,
}

// -------------------------------------------------------------------------------------------------

/// The [`Tick::PreTick`], [`Tick::Tick`], and [`Tick::PostTick`] schedules.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, ScheduleLabel)]
pub enum Tick {
    /// A [`Schedule`](bevy_ecs::schedule::Schedule) that runs before
    /// [`Tick::Tick`].
    PreTick,
    /// The main `Tick` [`Schedule`](bevy_ecs::schedule::Schedule).
    Tick,
    /// A [`Schedule`](bevy_ecs::schedule::Schedule) that runs after
    /// [`Tick::Tick`].
    PostTick,
}
