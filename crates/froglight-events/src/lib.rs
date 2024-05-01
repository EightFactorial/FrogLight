#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

use bevy_app::{App, Plugin};

pub mod events;
pub mod systemsets;

/// The `Events` Froglight plugin.
///
/// Adds events and systems for event handling.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EventPlugin;

impl Plugin for EventPlugin {
    fn build(&self, app: &mut App) {
        systemsets::build(app);
        events::build(app);
    }
}
