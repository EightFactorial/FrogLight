use bevy::app::{App, Plugin};

/// A plugin that adds a panorama to the interface.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct InterfacePanoramaPlugin;

impl Plugin for InterfacePanoramaPlugin {
    fn build(&self, app: &mut App) { super::build(app); }
}
