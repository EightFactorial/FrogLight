#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

use bevy_app::{App, Plugin};

/// The `Render` Froglight plugin.
///
/// Adds
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RenderPlugin;

impl Plugin for RenderPlugin {
    fn build(&self, _app: &mut App) {}
}
