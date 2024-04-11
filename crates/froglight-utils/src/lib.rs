#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![feature(const_type_name)]
#![feature(const_type_id)]
#![feature(associated_type_defaults)]

use bevy_app::{App, Plugin};

pub mod fixed_schedules;

/// The `Utility` Froglight plugin.
///
/// Adds various tools and utilities.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UtilityPlugin;

impl Plugin for UtilityPlugin {
    fn build(&self, app: &mut App) { fixed_schedules::build(app); }
}
