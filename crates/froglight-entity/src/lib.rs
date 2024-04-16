#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

use bevy_app::{App, Plugin};

/// The `Entity` Froglight plugin.
///
/// Adds Entity-related components and systems.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EntityPlugin;

impl Plugin for EntityPlugin {
    fn build(&self, _app: &mut App) {}
}

// TODO: Update entity's `ChunkPosition` in `EntityPostUpdateSchedule`
