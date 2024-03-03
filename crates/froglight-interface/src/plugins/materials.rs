use bevy::app::{App, Plugin};

/// A plugin that registers all custom materials
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MaterialPlugin;

impl Plugin for MaterialPlugin {
    fn build(&self, app: &mut App) { crate::materials::build(app); }

    fn finish(&self, app: &mut App) { crate::materials::finish(app); }
}
