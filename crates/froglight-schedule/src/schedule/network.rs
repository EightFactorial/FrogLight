use bevy_ecs::schedule::ScheduleLabel;

/// A [`ScheduleLabel`] for early-scheduled networking.
///
/// Typically used for receiving network messages.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, ScheduleLabel)]
pub struct PreNetwork;

/// A [`ScheduleLabel`] for late-scheduled networking.
///
/// Typically used for sending network messages.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, ScheduleLabel)]
pub struct PostNetwork;
