use bevy::prelude::*;

/// The [`SystemSet`] for [`froglight-debug`](crate) systems that run in the
/// [`Update`] schedule.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
pub struct DebugUpdateSet;
