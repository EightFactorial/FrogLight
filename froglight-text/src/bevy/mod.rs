//! TODO

use bevy_app::{App, Plugin};

/// A [`Plugin`] that adds text components and systems.
///
/// # Note
///
/// This plugin only adds systems when either the `bevy_ui` or `bevy_sprite`
/// features are enabled.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TextPlugin;

impl Plugin for TextPlugin {
    fn build(&self, _app: &mut App) {}
}
