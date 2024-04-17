#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

use bevy_app::{App, Plugin};

/// The `Settings` Froglight plugin.
///
/// Adds
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, _app: &mut App) {}
}
