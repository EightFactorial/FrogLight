#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

use bevy_app::{App, Plugin};

pub mod schedules;
pub mod tracking;

/// The `Utility` Froglight plugin.
///
/// Adds various tools and utilities.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UtilityPlugin;

impl Plugin for UtilityPlugin {
    fn build(&self, app: &mut App) {
        schedules::build(app);
        tracking::build(app);
    }
}
