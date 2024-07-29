#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

use bevy::app::{App, Plugin};

/// A [`Plugin`] that adds custom materials for rendering.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RenderPlugin;

impl Plugin for RenderPlugin {
    fn build(&self, _app: &mut App) {}
}
