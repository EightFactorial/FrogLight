use bevy::prelude::*;

/// A [`Plugin`] that shows a loading screen while assets are being loaded
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, _app: &mut App) {}
}
